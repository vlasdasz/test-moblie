//
//  test-engine.h
//  TestEngine
//
//  Created by Vladas Zakrevskis on 15/03/2024.
//


#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>


UITextField* text_field = nil;

void test_engine_show_alert(const char* message) {
    
    NSString* ns_message = [NSString stringWithUTF8String:message];
    
    UIAlertController *alertController = [UIAlertController alertControllerWithTitle:nil
                                                                             message:ns_message
                                                                      preferredStyle:UIAlertControllerStyleAlert];
    
    UIAlertAction *okAction = [UIAlertAction actionWithTitle:@"OK"
                                                       style:UIAlertActionStyleDefault
                                                     handler:^(UIAlertAction *action) {
                                                         // Handle the OK action if needed
                                                     }];
    
    [alertController addAction:okAction];
    
    UIViewController* controller = [UIApplication sharedApplication].keyWindow.rootViewController;
    [controller presentViewController:alertController animated:YES completion:nil];
}

void test_engine_ios_init_text_field(void) {
    NSLog(@"ios_init_text_field");

    assert(text_field == nil);
    UIViewController* controller = [UIApplication sharedApplication].keyWindow.rootViewController;
    text_field = [UITextField new];
    text_field.textAlignment = NSTextAlignmentCenter;
    [controller.view addSubview:text_field];
    
    NSLog(@"UITextField initialized");
}

void test_engine_ios_open_keyboard(float x, float y, float width, float height) {
    CGFloat scale = [[UIScreen mainScreen] scale];
    [text_field setFrame:CGRectMake(x / scale, y / scale, width / scale, height / scale)];
    [text_field setHidden:NO];
    NSLog(@"ios_open_keyboard");
    [text_field becomeFirstResponder];
}

const char* test_engine_ios_close_keyboard(void) {
    NSLog(@"ios_close_keyboard");
    [text_field resignFirstResponder];
    [text_field setHidden:YES];
    return [text_field.text UTF8String];
}
