#import <AppKit/AppKit.h>
#import <Foundation/Foundation.h>

@interface AppDelegate : NSObject <NSApplicationDelegate> {
  @public NSStatusItem *myStatusItem;
}

- (void)applicationDidFinishLaunching:(NSNotification *)aNotification;

@end