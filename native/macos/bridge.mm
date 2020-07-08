/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

#import <Foundation/Foundation.h>
#include <IOKit/IOKitLib.h>

#include "bridge.h"
#include "AppDelegate.h"

KeypressCallback keypress_callback;

void * context_instance;
AppDelegate * delegate_ptr;

int32_t initialize(void * context) {
    context_instance = context;

    AppDelegate *delegate = [[AppDelegate alloc] init];
    delegate_ptr = delegate;
    NSApplication * application = [NSApplication sharedApplication];
    [application setDelegate:delegate];
}

void register_keypress_callback(KeypressCallback callback) {
    keypress_callback = callback;
}

int32_t eventloop() {
    [NSApp run];
}

// 10.9+ only, see this url for compatibility:
// http://stackoverflow.com/questions/17693408/enable-access-for-assistive-devices-programmatically-on-10-9
int32_t check_accessibility() {
    NSDictionary *options = @{(id)kAXTrustedCheckOptionPrompt: @NO};
    BOOL accessibilityEnabled = AXIsProcessTrustedWithOptions((CFDictionaryRef)options);
}

int32_t prompt_accessibility() {
    NSDictionary* opts = @{(__bridge id)kAXTrustedCheckOptionPrompt: @YES};
    return AXIsProcessTrustedWithOptions((__bridge CFDictionaryRef)opts);
}

// Taken (with a few modifications) from the MagicKeys project: https://github.com/zsszatmari/MagicKeys
int32_t get_secure_input_process(int64_t *pid) {
    NSArray *consoleUsersArray;
    io_service_t rootService;
    int32_t result = 0;

    if ((rootService = IORegistryGetRootEntry(kIOMasterPortDefault)) != 0)
    {
        if ((consoleUsersArray = (NSArray *)IORegistryEntryCreateCFProperty((io_registry_entry_t)rootService, CFSTR("IOConsoleUsers"), kCFAllocatorDefault, 0)) != nil)
        {
            if ([consoleUsersArray isKindOfClass:[NSArray class]])  // Be careful - ensure this really is an array
            {
                for (NSDictionary *consoleUserDict in consoleUsersArray) {
                    NSNumber *secureInputPID;

                    if ((secureInputPID = [consoleUserDict objectForKey:@"kCGSSessionSecureInputPID"]) != nil)
                    {
                        if ([secureInputPID isKindOfClass:[NSNumber class]])
                        {
                            *pid = ((UInt64) [secureInputPID intValue]);
                            result = 1;
                            break;
                        }
                    }
                }
            }

            CFRelease((CFTypeRef)consoleUsersArray);
        }

        IOObjectRelease((io_object_t) rootService);
    }

    return result;
}
