// https://gist.github.com/bellbind/6954679
// capture image from webcam(e.g. face time)
// for OSX 10.9 (use AVFoundation API instead of deprecated QTKit)
// clang -fobjc-arc -Wall -Wextra -pedantic avcapture.m 
//    -framework Cocoa -framework AVFoundation -framework CoreMedia 
//    -framework QuartzCore -o avcapture

#import <AVFoundation/AVFoundation.h>
#import <AppKit/AppKit.h>

#include "avtest.h"

@interface Capture : NSObject <AVCaptureVideoDataOutputSampleBufferDelegate>
@property (weak) AVCaptureSession* session;
- (void) captureOutput: (AVCaptureOutput*) output
 didOutputSampleBuffer: (CMSampleBufferRef) buffer
        fromConnection: (AVCaptureConnection*) connection;
@end
@interface Capture ()
{
  CVImageBufferRef head;
  CFRunLoopRef runLoop;
  int count;
}
- (void) save;
@end

@implementation Capture
@synthesize session;

- (id) init
{
  self = [super init];
  runLoop = CFRunLoopGetCurrent();
  head = nil;
  count = 0;
  return self;
}

- (void) dealloc
{
  @synchronized (self) {
    CVBufferRelease(head);
  }
  NSLog(@"capture released");
}

- (void) save
{
  @synchronized (self) {
    CIImage* ciImage = [CIImage imageWithCVImageBuffer: head];
    NSBitmapImageRep* bitmapRep = [[NSBitmapImageRep alloc] initWithCIImage: ciImage];
    NSDictionary *imageProps = [NSDictionary dictionaryWithObject:[NSNumber numberWithFloat:1.0] forKey:NSImageCompressionFactor];    
    NSData* jpgData = [bitmapRep representationUsingType:NSBitmapImageFileTypeJPEG properties: imageProps];
    [jpgData writeToFile: @"result.jpg" atomically: NO];
    //NSData* pngData = 
    //  [bitmapRep representationUsingType:NSPNGFileType properties: nil];
    //[pngData writeToFile: @"result.png" atomically: NO];
  }
  NSLog(@"Saved");
}

- (void) captureOutput: (AVCaptureOutput*) output
   didOutputSampleBuffer: (CMSampleBufferRef) buffer
        fromConnection: (AVCaptureConnection*) connection 
{
#pragma unused (output)
#pragma unused (connection)

  CVImageBufferRef frame = CMSampleBufferGetImageBuffer(buffer);
  CVImageBufferRef prev;
  CVBufferRetain(frame);
  @synchronized (self) {
    prev = head;
    head = frame;
    count++;
    NSLog(@"Captured");
  }
  CVBufferRelease(prev);
  if (count > 10) {
    NSLog(@"decided to stop");
    // after skipped 5 frames
    [self save];
    [self.session stopRunning];
    CFRunLoopStop(runLoop);
  }
}
//- (void) captureOutput: (AVCaptureOutput*) output
//   didDropSampleBuffer: (CMSampleBufferRef) buffer
//        fromConnection: (AVCaptureConnection*) connection 
//{
//#pragma unused (output)
//#pragma unused (buffer)
//#pragma unused (connection)
//}
@end


int quit(NSError * error) {
  NSLog(@"[error] %@", [error localizedDescription]);
  return 1;
}

// HACK - make global to prevent from being deallocated when they leave scope
// later turn into some kind of class or utility widget
Capture* capture;
//AVCaptureDevice* device;
//AVCaptureDeviceInput* input;
//AVCaptureVideoDataOutput* output;
AVCaptureSession* session;

void avtest(void* _device, void* _input, void* _output)
{

  //AVCaptureDevice* device = (__bridge AVCaptureDevice*)_device;
  //AVCaptureDeviceInput* input = (__bridge AVCaptureDeviceInput*)_input;
  AVCaptureVideoDataOutput* output = (__bridge AVCaptureVideoDataOutput*)_output;

  NSLog(@"AVTest.m starting...");

  //just for fun
  //NSArray *captureDeviceType = @[AVCaptureDeviceTypeBuiltInWideAngleCamera];
  //AVCaptureDeviceDiscoverySession *captureDevice =
  //    [AVCaptureDeviceDiscoverySession
  //      discoverySessionWithDeviceTypes:captureDeviceType
  //      mediaType:AVMediaTypeVideo
  //      position:AVCaptureDevicePositionUnspecified
  //      ];
  //for(id object in captureDevice.devices) { NSLog(@"all devices %@",object); }

  //device = [AVCaptureDevice defaultDeviceWithMediaType: AVMediaTypeVideo];
  //NSLog(@"Got Device %@",device);
  //CFShow(CFBridgingRetain(device));

  //NSError* error = nil;
  //input = [AVCaptureDeviceInput deviceInputWithDevice: device  error: &error];
  //NSLog(@"Got Input");

  //output = [[AVCaptureVideoDataOutput alloc] init];
  capture = [[Capture alloc] init];
  [output setSampleBufferDelegate: capture queue: dispatch_get_main_queue()];
  NSLog(@"AVTest: Attached capture handler to output");
  
  //session = [[AVCaptureSession alloc] init];
  //[session addInput: input];
  //[session addOutput: output];
  
  //capture.session = session;
  //[session startRunning];

  NSLog(@"Started");

  //CFShow(buffer);   
  //CFTypeID blah = CFGetTypeID(buffer);
  //NSLog(@"something %@",blah);
  
}











