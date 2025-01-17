# Testapp

This is a simple test repo to create embeddings in Rust in the tauri backend. It works perfectly on MacOS. Fails however on iOS. Currently investigating.

## iOS steps

- npm run tauri ios init
- npm run tauri ios dev open  , dann einmal das Team einstellen

* In Xcode, open the src-tauri/gen/apple/places.xcodeproj/project.xcworkspace.
* Select your project (places_iOS).
* Go to the Signing & Capabilities tab.
* Make sure Automatically manage signing is enabled.
* Select your team from the team dropdown.

<img width="1061" alt="image" src="https://github.com/user-attachments/assets/ad5eae1d-9cdc-4ade-a2c7-faac8df1431e" />

Attention: if error here, check if the app identifier is gloablly unqiue. if it's not, change in tauri.conf.json! Run again.

- npm run tauri ios dev --host

## Build errors 

```shell
(base) ➜  places git:(main) ✗ cargo tauri ios dev --host  
Detected connected device: Dominik Marcus’s iPhone (iPhone16,1) with target "aarch64-apple-ios"
    Info Using 192.168.1.7 to access the development server.
Building app...
Command line invocation:
    /Applications/Xcode.app/Contents/Developer/usr/bin/xcodebuild -allowProvisioningUpdates -scheme places_iOS -workspace /Users/dome/work/general/tauri/places/src-tauri/gen/apple/places.xcodeproj/project.xcworkspace/ -sdk iphoneos -configuration debug build

User defaults from command line:
    IDEPackageSupportUseBuiltinSCM = YES

Build settings from command line:
    SDKROOT = iphoneos18.2

--- xcodebuild: WARNING: Using the first of multiple matching destinations:
{ platform:iOS, id:dvtdevice-DVTiPhonePlaceholder-iphoneos:placeholder, name:Any iOS Device }
{ platform:macOS, arch:arm64, variant:Designed for [iPad,iPhone], id:00006031-000820513402001C, name:My Mac }
{ platform:iOS Simulator, id:dvtdevice-DVTiOSDeviceSimulatorPlaceholder-iphonesimulator:placeholder, name:Any iOS Simulator Device }
{ platform:iOS Simulator, id:40464C3E-72F6-4C6D-9DD5-0E1DE9DC4992, OS:18.2, name:iPad (10th generation) }
{ platform:iOS Simulator, id:A55029F8-61BB-4F0A-A8D8-1AC631571C7E, OS:18.2, name:iPad Air 11-inch (M2) }
{ platform:iOS Simulator, id:ADFA96CB-47A1-4E17-97FC-CE9E3B796164, OS:18.2, name:iPad Air 13-inch (M2) }
{ platform:iOS Simulator, id:FBCA32A4-BD7E-4318-83D3-11BFE8E9E5DB, OS:18.2, name:iPad Pro 11-inch (M4) }
{ platform:iOS Simulator, id:3722BC7E-E29A-42AB-8535-A7BBB2AAD233, OS:18.2, name:iPad Pro 13-inch (M4) }
{ platform:iOS Simulator, id:1442FD98-4FB5-4517-9AC1-33EC7C1F4B49, OS:18.2, name:iPad mini (A17 Pro) }
{ platform:iOS Simulator, id:6EC2DF2D-8629-436F-A0B0-F6F6676FDB05, OS:18.2, name:iPhone 16 }
{ platform:iOS Simulator, id:4F35C27D-80C2-4E34-9B0B-F2DFA8A451BD, OS:18.2, name:iPhone 16 Plus }
{ platform:iOS Simulator, id:4577A2F8-0202-4B0A-B31F-7B7847B5DBC4, OS:18.2, name:iPhone 16 Pro }
{ platform:iOS Simulator, id:D5D68230-6F65-4290-B3BD-E960028FB326, OS:18.2, name:iPhone 16 Pro Max }
{ platform:iOS Simulator, id:46164BD7-EFF4-460D-B675-79D2B393F905, OS:18.2, name:iPhone SE (3rd generation) }
{ platform:iOS, arch:arm64, id:00008130-00014DD43C2B803A, name:Dominik Marcus’s iPhone }
Prepare packages

ComputeTargetDependencyGraph
note: Building targets in dependency order
note: Target dependency graph (1 target)
    Target 'places_iOS' in project 'places' (no dependencies)

GatherProvisioningInputs

CreateBuildDescription

ExecuteExternalTool /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang -v -E -dM -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk -x c -c /dev/null

ExecuteExternalTool /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang -v -E -dM -arch arm64 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk -x objective-c++ -c /dev/null

ExecuteExternalTool /Applications/Xcode.app/Contents/Developer/usr/bin/ibtool --version --output-format xml1

ExecuteExternalTool /Applications/Xcode.app/Contents/Developer/usr/bin/actool --print-asset-tag-combinations --output-format xml1 /Users/dome/work/general/tauri/places/src-tauri/gen/apple/Assets.xcassets

ExecuteExternalTool /Applications/Xcode.app/Contents/Developer/usr/bin/actool --version --output-format xml1

ExecuteExternalTool /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ld -version_details

ExecuteExternalTool /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang -v -E -dM -arch arm64 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk -x c -c /dev/null

Build description signature: f7f17c772e11ac964a8d02a11be84e75
Build description path: /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/XCBuildData/f7f17c772e11ac964a8d02a11be84e75.xcbuilddata
note: Run script build phase 'Build Rust Code' will be run during every build because the option to run the script phase "Based on dependency analysis" is unchecked. (in target 'places_iOS' from project 'places')
ProcessProductPackaging /Users/dome/Library/Developer/Xcode/UserData/Provisioning\ Profiles/5eb4f3ea-3632-405b-8ec7-e342940d3a69.mobileprovision /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos/places.app/embedded.mobileprovision (in target 'places_iOS' from project 'places')
    cd /Users/dome/work/general/tauri/places/src-tauri/gen/apple
    builtin-productPackagingUtility /Users/dome/Library/Developer/Xcode/UserData/Provisioning\ Profiles/5eb4f3ea-3632-405b-8ec7-e342940d3a69.mobileprovision -o /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos/places.app/embedded.mobileprovision

WriteAuxiliaryFile /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/DerivedSources/Entitlements.plist (in target 'places_iOS' from project 'places')
    cd /Users/dome/work/general/tauri/places/src-tauri/gen/apple
    write-file /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/DerivedSources/Entitlements.plist

ClangStatCache /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang-stat-cache /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk /Users/dome/Library/Developer/Xcode/DerivedData/SDKStatCaches.noindex/iphoneos18.2-22C146-d5b9239ec3bf5b3adbecdf21472871e3.sdkstatcache
    cd /Users/dome/work/general/tauri/places/src-tauri/gen/apple/places.xcodeproj
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang-stat-cache /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk -o /Users/dome/Library/Developer/Xcode/DerivedData/SDKStatCaches.noindex/iphoneos18.2-22C146-d5b9239ec3bf5b3adbecdf21472871e3.sdkstatcache
    Info connection; remote_addr=127.0.0.1:51115 conn_id=0

ProcessProductPackaging /Users/dome/work/general/tauri/places/src-tauri/gen/apple/places_iOS/places_iOS.entitlements /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/places.app.xcent (in target 'places_iOS' from project 'places')
    cd /Users/dome/work/general/tauri/places/src-tauri/gen/apple
    
    Entitlements:
    
    {
    "application-identifier" = "K7XMFG27ZR.com.my-tauri-app-with-errors1.app";
    "com.apple.developer.team-identifier" = K7XMFG27ZR;
    "get-task-allow" = 1;
}
    
    builtin-productPackagingUtility /Users/dome/work/general/tauri/places/src-tauri/gen/apple/places_iOS/places_iOS.entitlements -entitlements -format xml -o /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/places.app.xcent

ProcessProductPackagingDER /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/places.app.xcent /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/places.app.xcent.der (in target 'places_iOS' from project 'places')
    cd /Users/dome/work/general/tauri/places/src-tauri/gen/apple
    /usr/bin/derq query -f xml -i /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/places.app.xcent -o /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/places.app.xcent.der --raw

PhaseScriptExecution Build\ Rust\ Code /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Script-6079DD776A3DEEB7D06B4644.sh (in target 'places_iOS' from project 'places')
    cd /Users/dome/work/general/tauri/places/src-tauri/gen/apple
    export ACTION\=build
    export AD_HOC_CODE_SIGNING_ALLOWED\=NO
    export AGGREGATE_TRACKED_DOMAINS\=YES
    export ALLOW_BUILD_REQUEST_OVERRIDES\=NO
    export ALLOW_TARGET_PLATFORM_SPECIALIZATION\=NO
    export ALTERNATE_GROUP\=staff
    export ALTERNATE_MODE\=u+w,go-w,a+rX
    export ALTERNATE_OWNER\=dome
    export ALTERNATIVE_DISTRIBUTION_WEB\=NO
    export ALWAYS_EMBED_SWIFT_STANDARD_LIBRARIES\=YES
    export ALWAYS_SEARCH_USER_PATHS\=NO
    export ALWAYS_USE_SEPARATE_HEADERMAPS\=NO
    export APPLE_INTERNAL_DEVELOPER_DIR\=/AppleInternal/Developer
    export APPLE_INTERNAL_DIR\=/AppleInternal
    export APPLE_INTERNAL_DOCUMENTATION_DIR\=/AppleInternal/Documentation
    export APPLE_INTERNAL_LIBRARY_DIR\=/AppleInternal/Library
    export APPLE_INTERNAL_TOOLS\=/AppleInternal/Developer/Tools
    export APPLICATION_EXTENSION_API_ONLY\=NO
    export APPLY_RULES_IN_COPY_FILES\=NO
    export APPLY_RULES_IN_COPY_HEADERS\=NO
    export APP_SHORTCUTS_ENABLE_FLEXIBLE_MATCHING\=YES
    export ARCHS\=arm64
    export ARCHS_STANDARD\=arm64
    export ARCHS_STANDARD_32_64_BIT\=armv7\ arm64
    export ARCHS_STANDARD_32_BIT\=armv7
    export ARCHS_STANDARD_64_BIT\=arm64
    export ARCHS_STANDARD_INCLUDING_64_BIT\=arm64
    export ARCHS_UNIVERSAL_IPHONE_OS\=armv7\ arm64
    export ASSETCATALOG_COMPILER_APPICON_NAME\=AppIcon
    export ASSETCATALOG_COMPILER_GENERATE_ASSET_SYMBOLS\=YES
    export AUTOMATICALLY_MERGE_DEPENDENCIES\=NO
    export AVAILABLE_PLATFORMS\=appletvos\ appletvsimulator\ driverkit\ iphoneos\ iphonesimulator\ macosx\ watchos\ watchsimulator\ xros\ xrsimulator
    export AppIdentifierPrefix\=K7XMFG27ZR.
    export BITCODE_GENERATION_MODE\=marker
    export BUILD_ACTIVE_RESOURCES_ONLY\=NO
    export BUILD_COMPONENTS\=headers\ build
    export BUILD_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products
    export BUILD_LIBRARY_FOR_DISTRIBUTION\=NO
    export BUILD_ROOT\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products
    export BUILD_STYLE\=
    export BUILD_VARIANTS\=normal
    export BUILT_PRODUCTS_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos
    export BUNDLE_CONTENTS_FOLDER_PATH_deep\=Contents/
    export BUNDLE_EXECUTABLE_FOLDER_NAME_deep\=MacOS
    export BUNDLE_EXTENSIONS_FOLDER_PATH\=Extensions
    export BUNDLE_FORMAT\=shallow
    export BUNDLE_FRAMEWORKS_FOLDER_PATH\=Frameworks
    export BUNDLE_PLUGINS_FOLDER_PATH\=PlugIns
    export BUNDLE_PRIVATE_HEADERS_FOLDER_PATH\=PrivateHeaders
    export BUNDLE_PUBLIC_HEADERS_FOLDER_PATH\=Headers
    export CACHE_ROOT\=/var/folders/rw/g3mzqszj4wq9d6qwvc0dlrym0000gn/C/com.apple.DeveloperTools/16.2-16C5032a/Xcode
    export CCHROOT\=/var/folders/rw/g3mzqszj4wq9d6qwvc0dlrym0000gn/C/com.apple.DeveloperTools/16.2-16C5032a/Xcode
    export CHMOD\=/bin/chmod
    export CHOWN\=/usr/sbin/chown
    export CLANG_ANALYZER_NONNULL\=YES
    export CLANG_ANALYZER_NUMBER_OBJECT_CONVERSION\=YES_AGGRESSIVE
    export CLANG_CACHE_FINE_GRAINED_OUTPUTS\=YES
    export CLANG_CXX_LANGUAGE_STANDARD\=gnu++14
    export CLANG_CXX_LIBRARY\=libc++
    export CLANG_ENABLE_EXPLICIT_MODULES\=YES
    export CLANG_ENABLE_MODULES\=YES
    export CLANG_ENABLE_OBJC_ARC\=YES
    export CLANG_ENABLE_OBJC_WEAK\=YES
    export CLANG_MODULES_BUILD_SESSION_FILE\=/Users/dome/Library/Developer/Xcode/DerivedData/ModuleCache.noindex/Session.modulevalidation
    export CLANG_WARN_BLOCK_CAPTURE_AUTORELEASING\=YES
    export CLANG_WARN_BOOL_CONVERSION\=YES
    export CLANG_WARN_COMMA\=YES
    export CLANG_WARN_CONSTANT_CONVERSION\=YES
    export CLANG_WARN_DEPRECATED_OBJC_IMPLEMENTATIONS\=YES
    export CLANG_WARN_DIRECT_OBJC_ISA_USAGE\=YES_ERROR
    export CLANG_WARN_DOCUMENTATION_COMMENTS\=YES
    export CLANG_WARN_EMPTY_BODY\=YES
    export CLANG_WARN_ENUM_CONVERSION\=YES
    export CLANG_WARN_INFINITE_RECURSION\=YES
    export CLANG_WARN_INT_CONVERSION\=YES
    export CLANG_WARN_NON_LITERAL_NULL_CONVERSION\=YES
    export CLANG_WARN_OBJC_IMPLICIT_RETAIN_SELF\=YES
    export CLANG_WARN_OBJC_LITERAL_CONVERSION\=YES
    export CLANG_WARN_OBJC_ROOT_CLASS\=YES_ERROR
    export CLANG_WARN_QUOTED_INCLUDE_IN_FRAMEWORK_HEADER\=YES
    export CLANG_WARN_RANGE_LOOP_ANALYSIS\=YES
    export CLANG_WARN_STRICT_PROTOTYPES\=YES
    export CLANG_WARN_SUSPICIOUS_MOVE\=YES
    export CLANG_WARN_UNGUARDED_AVAILABILITY\=YES_AGGRESSIVE
    export CLANG_WARN_UNREACHABLE_CODE\=YES
    export CLANG_WARN__DUPLICATE_METHOD_MATCH\=YES
    export CLASS_FILE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/JavaClasses
    export CLEAN_PRECOMPS\=YES
    export CLONE_HEADERS\=NO
    export CODESIGNING_FOLDER_PATH\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos/places.app
    export CODE_SIGNING_ALLOWED\=YES
    export CODE_SIGNING_REQUIRED\=YES
    export CODE_SIGN_CONTEXT_CLASS\=XCiPhoneOSCodeSignContext
    export CODE_SIGN_ENTITLEMENTS\=places_iOS/places_iOS.entitlements
    export CODE_SIGN_IDENTITY\=iPhone\ Developer
    export CODE_SIGN_INJECT_BASE_ENTITLEMENTS\=YES
    export COLOR_DIAGNOSTICS\=YES
    export COMBINE_HIDPI_IMAGES\=NO
    export COMPILATION_CACHE_CAS_PATH\=/Users/dome/Library/Developer/Xcode/DerivedData/CompilationCache.noindex
    export COMPILATION_CACHE_KEEP_CAS_DIRECTORY\=YES
    export COMPILER_INDEX_STORE_ENABLE\=Default
    export COMPOSITE_SDK_DIRS\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/CompositeSDKs
    export COMPRESS_PNG_FILES\=YES
    export CONFIGURATION\=debug
    export CONFIGURATION_BUILD_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos
    export CONFIGURATION_TEMP_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos
    export CONTENTS_FOLDER_PATH\=places.app
    export CONTENTS_FOLDER_PATH_SHALLOW_BUNDLE_NO\=places.app/Contents
    export CONTENTS_FOLDER_PATH_SHALLOW_BUNDLE_YES\=places.app
    export COPYING_PRESERVES_HFS_DATA\=NO
    export COPY_HEADERS_RUN_UNIFDEF\=NO
    export COPY_PHASE_STRIP\=NO
    export CORRESPONDING_SIMULATOR_PLATFORM_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform
    export CORRESPONDING_SIMULATOR_PLATFORM_NAME\=iphonesimulator
    export CORRESPONDING_SIMULATOR_SDK_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator18.2.sdk
    export CORRESPONDING_SIMULATOR_SDK_NAME\=iphonesimulator18.2
    export CP\=/bin/cp
    export CREATE_INFOPLIST_SECTION_IN_BINARY\=NO
    export CURRENT_ARCH\=undefined_arch
    export CURRENT_VARIANT\=normal
    export DEAD_CODE_STRIPPING\=YES
    export DEBUGGING_SYMBOLS\=YES
    export DEBUG_INFORMATION_FORMAT\=dwarf
    export DEBUG_INFORMATION_VERSION\=compiler-default
    export DEFAULT_COMPILER\=com.apple.compilers.llvm.clang.1_0
    export DEFAULT_DEXT_INSTALL_PATH\=/System/Library/DriverExtensions
    export DEFAULT_KEXT_INSTALL_PATH\=/System/Library/Extensions
    export DEFINES_MODULE\=NO
    export DEPLOYMENT_LOCATION\=NO
    export DEPLOYMENT_POSTPROCESSING\=NO
    export DEPLOYMENT_TARGET_SETTING_NAME\=IPHONEOS_DEPLOYMENT_TARGET
    export DEPLOYMENT_TARGET_SUGGESTED_VALUES\=12.0\ 12.1\ 12.2\ 12.3\ 12.4\ 13.0\ 13.1\ 13.2\ 13.3\ 13.4\ 13.5\ 13.6\ 14.0\ 14.1\ 14.2\ 14.3\ 14.4\ 14.5\ 14.6\ 14.7\ 15.0\ 15.1\ 15.2\ 15.3\ 15.4\ 15.5\ 15.6\ 16.0\ 16.1\ 16.2\ 16.3\ 16.4\ 16.5\ 16.6\ 17.0\ 17.1\ 17.2\ 17.3\ 17.4\ 17.5\ 17.6\ 18.0\ 18.1\ 18.2
    export DERIVED_FILES_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/DerivedSources
    export DERIVED_FILE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/DerivedSources
    export DERIVED_SOURCES_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/DerivedSources
    export DERIVE_MACCATALYST_PRODUCT_BUNDLE_IDENTIFIER\=NO
    export DEVELOPER_APPLICATIONS_DIR\=/Applications/Xcode.app/Contents/Developer/Applications
    export DEVELOPER_BIN_DIR\=/Applications/Xcode.app/Contents/Developer/usr/bin
    export DEVELOPER_DIR\=/Applications/Xcode.app/Contents/Developer
    export DEVELOPER_FRAMEWORKS_DIR\=/Applications/Xcode.app/Contents/Developer/Library/Frameworks
    export DEVELOPER_FRAMEWORKS_DIR_QUOTED\=/Applications/Xcode.app/Contents/Developer/Library/Frameworks
    export DEVELOPER_LIBRARY_DIR\=/Applications/Xcode.app/Contents/Developer/Library
    export DEVELOPER_SDK_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs
    export DEVELOPER_TOOLS_DIR\=/Applications/Xcode.app/Contents/Developer/Tools
    export DEVELOPER_USR_DIR\=/Applications/Xcode.app/Contents/Developer/usr
    export DEVELOPMENT_LANGUAGE\=en
    export DEVELOPMENT_TEAM\=K7XMFG27ZR
    export DIFF\=/usr/bin/diff
    export DOCUMENTATION_FOLDER_PATH\=places.app/en.lproj/Documentation
    export DONT_GENERATE_INFOPLIST_FILE\=NO
    export DSTROOT\=/tmp/places.dst
    export DT_TOOLCHAIN_DIR\=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain
    export DWARF_DSYM_FILE_NAME\=places.app.dSYM
    export DWARF_DSYM_FILE_SHOULD_ACCOMPANY_PRODUCT\=NO
    export DWARF_DSYM_FOLDER_PATH\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos
    export DYNAMIC_LIBRARY_EXTENSION\=dylib
    export EAGER_COMPILATION_ALLOW_SCRIPTS\=NO
    export EAGER_LINKING\=NO
    export EFFECTIVE_PLATFORM_NAME\=-iphoneos
    export EMBEDDED_CONTENT_CONTAINS_SWIFT\=NO
    export EMBEDDED_PROFILE_NAME\=embedded.mobileprovision
    export EMBED_ASSET_PACKS_IN_PRODUCT_BUNDLE\=NO
    export ENABLE_APP_SANDBOX\=NO
    export ENABLE_BITCODE\=NO
    export ENABLE_CODE_COVERAGE\=YES
    export ENABLE_DEBUG_DYLIB\=YES
    export ENABLE_DEFAULT_HEADER_SEARCH_PATHS\=YES
    export ENABLE_DEFAULT_SEARCH_PATHS\=YES
    export ENABLE_HARDENED_RUNTIME\=NO
    export ENABLE_HEADER_DEPENDENCIES\=YES
    export ENABLE_ON_DEMAND_RESOURCES\=YES
    export ENABLE_PREVIEWS\=NO
    export ENABLE_STRICT_OBJC_MSGSEND\=YES
    export ENABLE_TESTABILITY\=YES
    export ENABLE_TESTING_SEARCH_PATHS\=NO
    export ENABLE_USER_SCRIPT_SANDBOXING\=NO
    export ENABLE_XOJIT_PREVIEWS\=YES
    export ENTITLEMENTS_ALLOWED\=YES
    export ENTITLEMENTS_DESTINATION\=Signature
    export ENTITLEMENTS_REQUIRED\=NO
    export EXCLUDED_ARCHS\=arm64-sim\ x86_64
    export EXCLUDED_INSTALLSRC_SUBDIRECTORY_PATTERNS\=.DS_Store\ .svn\ .git\ .hg\ CVS
    export EXCLUDED_RECURSIVE_SEARCH_PATH_SUBDIRECTORIES\=\*.nib\ \*.lproj\ \*.framework\ \*.gch\ \*.xcode\*\ \*.xcassets\ \(\*\)\ .DS_Store\ CVS\ .svn\ .git\ .hg\ \*.pbproj\ \*.pbxproj
    export EXECUTABLES_FOLDER_PATH\=places.app/Executables
    export EXECUTABLE_BLANK_INJECTION_DYLIB_PATH\=places.app/__preview.dylib
    export EXECUTABLE_DEBUG_DYLIB_INSTALL_NAME\=@rpath/places.debug.dylib
    export EXECUTABLE_DEBUG_DYLIB_PATH\=places.app/places.debug.dylib
    export EXECUTABLE_FOLDER_PATH\=places.app
    export EXECUTABLE_FOLDER_PATH_SHALLOW_BUNDLE_NO\=places.app/MacOS
    export EXECUTABLE_FOLDER_PATH_SHALLOW_BUNDLE_YES\=places.app
    export EXECUTABLE_NAME\=places
    export EXECUTABLE_PATH\=places.app/places
    export EXPANDED_CODE_SIGN_IDENTITY\=DFBE83B07DD5A890BB0A2B0B7E3FCBE1D9EBBAB2
    export EXPANDED_CODE_SIGN_IDENTITY_NAME\=Apple\ Development:\ d_weckmueller@web.de\ \(3CGH36XW53\)
    export EXPANDED_PROVISIONING_PROFILE\=5eb4f3ea-3632-405b-8ec7-e342940d3a69
    export EXTENSIONS_FOLDER_PATH\=places.app/Extensions
    export FILE_LIST\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects/LinkFileList
    export FIXED_FILES_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/FixedFiles
    export FRAMEWORKS_FOLDER_PATH\=places.app/Frameworks
    export FRAMEWORK_FLAG_PREFIX\=-framework
    export FRAMEWORK_SEARCH_PATHS\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos\ \ \".\"
    export FRAMEWORK_VERSION\=A
    export FULL_PRODUCT_NAME\=places.app
    export FUSE_BUILD_PHASES\=YES
    export FUSE_BUILD_SCRIPT_PHASES\=NO
    export GCC3_VERSION\=3.3
    export GCC_C_LANGUAGE_STANDARD\=gnu11
    export GCC_DYNAMIC_NO_PIC\=NO
    export GCC_INLINES_ARE_PRIVATE_EXTERN\=YES
    export GCC_NO_COMMON_BLOCKS\=YES
    export GCC_OPTIMIZATION_LEVEL\=0
    export GCC_PFE_FILE_C_DIALECTS\=c\ objective-c\ c++\ objective-c++
    export GCC_PREPROCESSOR_DEFINITIONS\=\ DEBUG\=1
    export GCC_SYMBOLS_PRIVATE_EXTERN\=NO
    export GCC_THUMB_SUPPORT\=YES
    export GCC_TREAT_WARNINGS_AS_ERRORS\=NO
    export GCC_VERSION\=com.apple.compilers.llvm.clang.1_0
    export GCC_VERSION_IDENTIFIER\=com_apple_compilers_llvm_clang_1_0
    export GCC_WARN_64_TO_32_BIT_CONVERSION\=YES
    export GCC_WARN_ABOUT_RETURN_TYPE\=YES_ERROR
    export GCC_WARN_UNDECLARED_SELECTOR\=YES
    export GCC_WARN_UNINITIALIZED_AUTOS\=YES_AGGRESSIVE
    export GCC_WARN_UNUSED_FUNCTION\=YES
    export GCC_WARN_UNUSED_VARIABLE\=YES
    export GENERATED_MODULEMAP_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/GeneratedModuleMaps-iphoneos
    export GENERATE_INFOPLIST_FILE\=NO
    export GENERATE_INTERMEDIATE_TEXT_BASED_STUBS\=YES
    export GENERATE_MASTER_OBJECT_FILE\=NO
    export GENERATE_PKGINFO_FILE\=YES
    export GENERATE_PROFILING_CODE\=NO
    export GENERATE_TEXT_BASED_STUBS\=NO
    export GID\=20
    export GROUP\=staff
    export HEADERMAP_INCLUDES_FLAT_ENTRIES_FOR_TARGET_BEING_BUILT\=YES
    export HEADERMAP_INCLUDES_FRAMEWORK_ENTRIES_FOR_ALL_PRODUCT_TYPES\=YES
    export HEADERMAP_INCLUDES_FRAMEWORK_ENTRIES_FOR_TARGETS_NOT_BEING_BUILT\=YES
    export HEADERMAP_INCLUDES_NONPUBLIC_NONPRIVATE_HEADERS\=YES
    export HEADERMAP_INCLUDES_PROJECT_HEADERS\=YES
    export HEADERMAP_USES_FRAMEWORK_PREFIX_ENTRIES\=YES
    export HEADERMAP_USES_VFS\=NO
    export HEADER_SEARCH_PATHS\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos/include\ 
    export HIDE_BITCODE_SYMBOLS\=YES
    export HOME\=/Users/dome
    export HOST_ARCH\=arm64
    export HOST_PLATFORM\=macosx
    export ICONV\=/usr/bin/iconv
    export IMPLICIT_DEPENDENCY_DOMAIN\=default
    export INFOPLIST_ENABLE_CFBUNDLEICONS_MERGE\=YES
    export INFOPLIST_EXPAND_BUILD_SETTINGS\=YES
    export INFOPLIST_FILE\=places_iOS/Info.plist
    export INFOPLIST_OUTPUT_FORMAT\=binary
    export INFOPLIST_PATH\=places.app/Info.plist
    export INFOPLIST_PREPROCESS\=NO
    export INFOSTRINGS_PATH\=places.app/en.lproj/InfoPlist.strings
    export INLINE_PRIVATE_FRAMEWORKS\=NO
    export INSTALLHDRS_COPY_PHASE\=NO
    export INSTALLHDRS_SCRIPT_PHASE\=NO
    export INSTALL_DIR\=/tmp/places.dst/Applications
    export INSTALL_GROUP\=staff
    export INSTALL_MODE_FLAG\=u+w,go-w,a+rX
    export INSTALL_OWNER\=dome
    export INSTALL_PATH\=/Applications
    export INSTALL_ROOT\=/tmp/places.dst
    export IPHONEOS_DEPLOYMENT_TARGET\=13.0
    export IS_UNOPTIMIZED_BUILD\=YES
    export JAVAC_DEFAULT_FLAGS\=-J-Xms64m\ -J-XX:NewSize\=4M\ -J-Dfile.encoding\=UTF8
    export JAVA_APP_STUB\=/System/Library/Frameworks/JavaVM.framework/Resources/MacOS/JavaApplicationStub
    export JAVA_ARCHIVE_CLASSES\=YES
    export JAVA_ARCHIVE_TYPE\=JAR
    export JAVA_COMPILER\=/usr/bin/javac
    export JAVA_FOLDER_PATH\=places.app/Java
    export JAVA_FRAMEWORK_RESOURCES_DIRS\=Resources
    export JAVA_JAR_FLAGS\=cv
    export JAVA_SOURCE_SUBDIR\=.
    export JAVA_USE_DEPENDENCIES\=YES
    export JAVA_ZIP_FLAGS\=-urg
    export JIKES_DEFAULT_FLAGS\=+E\ +OLDCSO
    export KASAN_CFLAGS_CLASSIC\=-DKASAN\=1\ -DKASAN_CLASSIC\=1\ -fsanitize\=address\ -mllvm\ -asan-globals-live-support\ -mllvm\ -asan-force-dynamic-shadow
    export KASAN_CFLAGS_TBI\=-DKASAN\=1\ -DKASAN_TBI\=1\ -fsanitize\=kernel-hwaddress\ -mllvm\ -hwasan-recover\=0\ -mllvm\ -hwasan-instrument-atomics\=0\ -mllvm\ -hwasan-instrument-stack\=1\ -mllvm\ -hwasan-generate-tags-with-calls\=1\ -mllvm\ -hwasan-instrument-with-calls\=1\ -mllvm\ -hwasan-use-short-granules\=0\ -mllvm\ -hwasan-memory-access-callback-prefix\=__asan_
    export KASAN_DEFAULT_CFLAGS\=-DKASAN\=1\ -DKASAN_CLASSIC\=1\ -fsanitize\=address\ -mllvm\ -asan-globals-live-support\ -mllvm\ -asan-force-dynamic-shadow
    export KEEP_PRIVATE_EXTERNS\=NO
    export LD_DEPENDENCY_INFO_FILE\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal/undefined_arch/places_dependency_info.dat
    export LD_EXPORT_SYMBOLS\=YES
    export LD_GENERATE_MAP_FILE\=NO
    export LD_MAP_FILE_PATH\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/places-LinkMap-normal-undefined_arch.txt
    export LD_NO_PIE\=NO
    export LD_QUOTE_LINKER_ARGUMENTS_FOR_COMPILER_DRIVER\=YES
    export LD_RUNPATH_SEARCH_PATHS\=\ @executable_path/Frameworks
    export LD_RUNPATH_SEARCH_PATHS_YES\=@loader_path/../Frameworks
    export LD_SHARED_CACHE_ELIGIBLE\=Automatic
    export LD_WARN_DUPLICATE_LIBRARIES\=NO
    export LD_WARN_UNUSED_DYLIBS\=NO
    export LEGACY_DEVELOPER_DIR\=/Applications/Xcode.app/Contents/PlugIns/Xcode3Core.ideplugin/Contents/SharedSupport/Developer
    export LEX\=lex
    export LIBRARY_DEXT_INSTALL_PATH\=/Library/DriverExtensions
    export LIBRARY_FLAG_NOSPACE\=YES
    export LIBRARY_FLAG_PREFIX\=-l
    export LIBRARY_KEXT_INSTALL_PATH\=/Library/Extensions
    export LIBRARY_SEARCH_PATHS\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos\ 
    export LINKER_DISPLAYS_MANGLED_NAMES\=NO
    export LINK_FILE_LIST_normal_arm64\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal/arm64/places.LinkFileList
    export LINK_OBJC_RUNTIME\=YES
    export LINK_WITH_STANDARD_LIBRARIES\=YES
    export LLVM_TARGET_TRIPLE_OS_VERSION\=ios13.0
    export LLVM_TARGET_TRIPLE_VENDOR\=apple
    export LM_AUX_CONST_METADATA_LIST_PATH_normal_arm64\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal/arm64/places.SwiftConstValuesFileList
    export LOCALIZATION_EXPORT_SUPPORTED\=YES
    export LOCALIZATION_PREFERS_STRING_CATALOGS\=NO
    export LOCALIZED_RESOURCES_FOLDER_PATH\=places.app/en.lproj
    export LOCALIZED_STRING_MACRO_NAMES\=NSLocalizedString\ CFCopyLocalizedString
    export LOCALIZED_STRING_SWIFTUI_SUPPORT\=YES
    export LOCAL_ADMIN_APPS_DIR\=/Applications/Utilities
    export LOCAL_APPS_DIR\=/Applications
    export LOCAL_DEVELOPER_DIR\=/Library/Developer
    export LOCAL_LIBRARY_DIR\=/Library
    export LOCROOT\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple
    export LOCSYMROOT\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple
    export MACH_O_TYPE\=mh_execute
    export MAC_OS_X_PRODUCT_BUILD_VERSION\=24B91
    export MAC_OS_X_VERSION_ACTUAL\=150101
    export MAC_OS_X_VERSION_MAJOR\=150000
    export MAC_OS_X_VERSION_MINOR\=150100
    export MAKE_MERGEABLE\=NO
    export MERGEABLE_LIBRARY\=NO
    export MERGED_BINARY_TYPE\=none
    export MERGE_LINKED_LIBRARIES\=NO
    export METAL_LIBRARY_FILE_BASE\=default
    export METAL_LIBRARY_OUTPUT_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos/places.app
    export MODULES_FOLDER_PATH\=places.app/Modules
    export MODULE_CACHE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/ModuleCache.noindex
    export MTL_ENABLE_DEBUG_INFO\=INCLUDE_SOURCE
    export MTL_FAST_MATH\=YES
    export NATIVE_ARCH\=arm64
    export NATIVE_ARCH_32_BIT\=arm
    export NATIVE_ARCH_64_BIT\=arm64
    export NATIVE_ARCH_ACTUAL\=arm64
    export NO_COMMON\=YES
    export OBJECT_FILE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects
    export OBJECT_FILE_DIR_normal\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal
    export OBJROOT\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex
    export ONLY_ACTIVE_ARCH\=NO
    export OS\=MACOS
    export OSAC\=/usr/bin/osacompile
    export PACKAGE_TYPE\=com.apple.package-type.wrapper.application
    export PASCAL_STRINGS\=YES
    export PATH\=/Applications/Xcode.app/Contents/SharedFrameworks/XCBuild.framework/Versions/A/PlugIns/XCBBuildService.bundle/Contents/PlugIns/XCBSpecifications.ideplugin/Contents/Resources:/Applications/Xcode.app/Contents/SharedFrameworks/XCBuild.framework/Versions/A/PlugIns/XCBBuildService.bundle/Contents/PlugIns/XCBSpecifications.ideplugin:/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin:/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/appleinternal/bin:/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/local/bin:/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/libexec:/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/usr/bin:/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/usr/appleinternal/bin:/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/usr/local/bin:/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/bin:/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/local/bin:/Applications/Xcode.app/Contents/Developer/usr/bin:/Applications/Xcode.app/Contents/Developer/usr/local/bin:/Users/dome/.nvm/versions/node/v20.10.0/bin:/opt/homebrew/bin:/opt/homebrew/sbin:/usr/local/bin:/System/Cryptexes/App/usr/bin:/usr/bin:/bin:/usr/sbin:/sbin:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/local/bin:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/bin:/var/run/com.apple.security.cryptexd/codex.system/bootstrap/usr/appleinternal/bin:/Library/Apple/usr/bin:/opt/homebrew/Caskroom/miniconda/base/bin:/opt/homebrew/Caskroom/miniconda/base/condabin:/Users/dome/.cargo/bin:/Users/dome/go/bin:/Users/dome/Library/Application\ Support/Code/User/globalStorage/github.copilot-chat/debugCommand
    export PATH_PREFIXES_EXCLUDED_FROM_HEADER_DEPENDENCIES\=/usr/include\ /usr/local/include\ /System/Library/Frameworks\ /System/Library/PrivateFrameworks\ /Applications/Xcode.app/Contents/Developer/Headers\ /Applications/Xcode.app/Contents/Developer/SDKs\ /Applications/Xcode.app/Contents/Developer/Platforms
    export PBDEVELOPMENTPLIST_PATH\=places.app/pbdevelopment.plist
    export PER_ARCH_OBJECT_FILE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal/undefined_arch
    export PER_VARIANT_OBJECT_FILE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal
    export PKGINFO_FILE_PATH\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/PkgInfo
    export PKGINFO_PATH\=places.app/PkgInfo
    export PLATFORM_DEVELOPER_APPLICATIONS_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/Applications
    export PLATFORM_DEVELOPER_BIN_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/bin
    export PLATFORM_DEVELOPER_LIBRARY_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/Library
    export PLATFORM_DEVELOPER_SDK_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs
    export PLATFORM_DEVELOPER_TOOLS_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/Tools
    export PLATFORM_DEVELOPER_USR_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr
    export PLATFORM_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform
    export PLATFORM_DISPLAY_NAME\=iOS
    export PLATFORM_FAMILY_NAME\=iOS
    export PLATFORM_NAME\=iphoneos
    export PLATFORM_PREFERRED_ARCH\=arm64
    export PLATFORM_PRODUCT_BUILD_VERSION\=22C146
    export PLIST_FILE_OUTPUT_FORMAT\=binary
    export PLUGINS_FOLDER_PATH\=places.app/PlugIns
    export PRECOMPS_INCLUDE_HEADERS_FROM_BUILT_PRODUCTS_DIR\=YES
    export PRECOMP_DESTINATION_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/PrefixHeaders
    export PRIVATE_HEADERS_FOLDER_PATH\=places.app/PrivateHeaders
    export PROCESSED_INFOPLIST_PATH\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal/undefined_arch/Processed-Info.plist
    export PRODUCT_BUNDLE_IDENTIFIER\=com.my-tauri-app-with-errors1.app
    export PRODUCT_BUNDLE_PACKAGE_TYPE\=APPL
    export PRODUCT_MODULE_NAME\=places
    export PRODUCT_NAME\=places
    export PRODUCT_SETTINGS_PATH\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple/places_iOS/Info.plist
    export PRODUCT_TYPE\=com.apple.product-type.application
    export PROFILING_CODE\=NO
    export PROJECT\=places
    export PROJECT_DERIVED_FILE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/DerivedSources
    export PROJECT_DIR\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple
    export PROJECT_FILE_PATH\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple/places.xcodeproj
    export PROJECT_GUID\=14a86506a56eb18b25d41e9dce5b262c
    export PROJECT_NAME\=places
    export PROJECT_TEMP_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build
    export PROJECT_TEMP_ROOT\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex
    export PROVISIONING_PROFILE_REQUIRED\=YES
    export PROVISIONING_PROFILE_REQUIRED_YES_YES\=YES
    export PROVISIONING_PROFILE_SUPPORTED\=YES
    export PUBLIC_HEADERS_FOLDER_PATH\=places.app/Headers
    export RECOMMENDED_IPHONEOS_DEPLOYMENT_TARGET\=15.0
    export RECURSIVE_SEARCH_PATHS_FOLLOW_SYMLINKS\=YES
    export REMOVE_CVS_FROM_RESOURCES\=YES
    export REMOVE_GIT_FROM_RESOURCES\=YES
    export REMOVE_HEADERS_FROM_EMBEDDED_BUNDLES\=YES
    export REMOVE_HG_FROM_RESOURCES\=YES
    export REMOVE_STATIC_EXECUTABLES_FROM_EMBEDDED_BUNDLES\=YES
    export REMOVE_SVN_FROM_RESOURCES\=YES
    export RESCHEDULE_INDEPENDENT_HEADERS_PHASES\=YES
    export REZ_COLLECTOR_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/ResourceManagerResources
    export REZ_OBJECTS_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/ResourceManagerResources/Objects
    export REZ_SEARCH_PATHS\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos\ 
    export SCAN_ALL_SOURCE_FILES_FOR_INCLUDES\=NO
    export SCRIPTS_FOLDER_PATH\=places.app/Scripts
    export SCRIPT_INPUT_FILE_COUNT\=0
    export SCRIPT_INPUT_FILE_LIST_COUNT\=0
    export SCRIPT_OUTPUT_FILE_0\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple/Externals/x86_64/debug/libapp.a
    export SCRIPT_OUTPUT_FILE_1\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple/Externals/arm64/debug/libapp.a
    export SCRIPT_OUTPUT_FILE_2\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple/Externals/arm64-sim/debug/libapp.a
    export SCRIPT_OUTPUT_FILE_COUNT\=3
    export SCRIPT_OUTPUT_FILE_LIST_COUNT\=0
    export SDKROOT\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk
    export SDK_DIR\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk
    export SDK_DIR_iphoneos\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk
    export SDK_DIR_iphoneos18_2\=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk
    export SDK_NAME\=iphoneos18.2
    export SDK_NAMES\=iphoneos18.2
    export SDK_PRODUCT_BUILD_VERSION\=22C146
    export SDK_STAT_CACHE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData
    export SDK_STAT_CACHE_ENABLE\=YES
    export SDK_STAT_CACHE_PATH\=/Users/dome/Library/Developer/Xcode/DerivedData/SDKStatCaches.noindex/iphoneos18.2-22C146-d5b9239ec3bf5b3adbecdf21472871e3.sdkstatcache
    export SDK_VERSION\=18.2
    export SDK_VERSION_ACTUAL\=180200
    export SDK_VERSION_MAJOR\=180000
    export SDK_VERSION_MINOR\=180200
    export SED\=/usr/bin/sed
    export SEPARATE_STRIP\=NO
    export SEPARATE_SYMBOL_EDIT\=NO
    export SET_DIR_MODE_OWNER_GROUP\=YES
    export SET_FILE_MODE_OWNER_GROUP\=NO
    export SHALLOW_BUNDLE\=YES
    export SHALLOW_BUNDLE_TRIPLE\=ios
    export SHALLOW_BUNDLE_ios_macabi\=NO
    export SHALLOW_BUNDLE_macos\=NO
    export SHARED_DERIVED_FILE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos/DerivedSources
    export SHARED_FRAMEWORKS_FOLDER_PATH\=places.app/SharedFrameworks
    export SHARED_PRECOMPS_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/PrecompiledHeaders
    export SHARED_SUPPORT_FOLDER_PATH\=places.app/SharedSupport
    export SKIP_INSTALL\=NO
    export SOURCE_ROOT\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple
    export SRCROOT\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple
    export STRINGSDATA_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal/undefined_arch
    export STRINGSDATA_ROOT\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build
    export STRINGS_FILE_INFOPLIST_RENAME\=YES
    export STRINGS_FILE_OUTPUT_ENCODING\=binary
    export STRIP_BITCODE_FROM_COPIED_FILES\=YES
    export STRIP_INSTALLED_PRODUCT\=NO
    export STRIP_STYLE\=all
    export STRIP_SWIFT_SYMBOLS\=YES
    export SUPPORTED_DEVICE_FAMILIES\=1,2
    export SUPPORTED_PLATFORMS\=iphoneos\ iphonesimulator
    export SUPPORTS_MACCATALYST\=NO
    export SUPPORTS_MAC_DESIGNED_FOR_IPHONE_IPAD\=YES
    export SUPPORTS_ON_DEMAND_RESOURCES\=YES
    export SUPPORTS_TEXT_BASED_API\=NO
    export SUPPORTS_XR_DESIGNED_FOR_IPHONE_IPAD\=YES
    export SUPPRESS_WARNINGS\=NO
    export SWIFT_ACTIVE_COMPILATION_CONDITIONS\=DEBUG
    export SWIFT_EMIT_LOC_STRINGS\=NO
    export SWIFT_OPTIMIZATION_LEVEL\=-Onone
    export SWIFT_PLATFORM_TARGET_PREFIX\=ios
    export SWIFT_RESPONSE_FILE_PATH_normal_arm64\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Objects-normal/arm64/places.SwiftFileList
    export SWIFT_VERSION\=5.0
    export SYMROOT\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products
    export SYSTEM_ADMIN_APPS_DIR\=/Applications/Utilities
    export SYSTEM_APPS_DIR\=/Applications
    export SYSTEM_CORE_SERVICES_DIR\=/System/Library/CoreServices
    export SYSTEM_DEMOS_DIR\=/Applications/Extras
    export SYSTEM_DEVELOPER_APPS_DIR\=/Applications/Xcode.app/Contents/Developer/Applications
    export SYSTEM_DEVELOPER_BIN_DIR\=/Applications/Xcode.app/Contents/Developer/usr/bin
    export SYSTEM_DEVELOPER_DEMOS_DIR\=/Applications/Xcode.app/Contents/Developer/Applications/Utilities/Built\ Examples
    export SYSTEM_DEVELOPER_DIR\=/Applications/Xcode.app/Contents/Developer
    export SYSTEM_DEVELOPER_DOC_DIR\=/Applications/Xcode.app/Contents/Developer/ADC\ Reference\ Library
    export SYSTEM_DEVELOPER_GRAPHICS_TOOLS_DIR\=/Applications/Xcode.app/Contents/Developer/Applications/Graphics\ Tools
    export SYSTEM_DEVELOPER_JAVA_TOOLS_DIR\=/Applications/Xcode.app/Contents/Developer/Applications/Java\ Tools
    export SYSTEM_DEVELOPER_PERFORMANCE_TOOLS_DIR\=/Applications/Xcode.app/Contents/Developer/Applications/Performance\ Tools
    export SYSTEM_DEVELOPER_RELEASENOTES_DIR\=/Applications/Xcode.app/Contents/Developer/ADC\ Reference\ Library/releasenotes
    export SYSTEM_DEVELOPER_TOOLS\=/Applications/Xcode.app/Contents/Developer/Tools
    export SYSTEM_DEVELOPER_TOOLS_DOC_DIR\=/Applications/Xcode.app/Contents/Developer/ADC\ Reference\ Library/documentation/DeveloperTools
    export SYSTEM_DEVELOPER_TOOLS_RELEASENOTES_DIR\=/Applications/Xcode.app/Contents/Developer/ADC\ Reference\ Library/releasenotes/DeveloperTools
    export SYSTEM_DEVELOPER_USR_DIR\=/Applications/Xcode.app/Contents/Developer/usr
    export SYSTEM_DEVELOPER_UTILITIES_DIR\=/Applications/Xcode.app/Contents/Developer/Applications/Utilities
    export SYSTEM_DEXT_INSTALL_PATH\=/System/Library/DriverExtensions
    export SYSTEM_DOCUMENTATION_DIR\=/Library/Documentation
    export SYSTEM_EXTENSIONS_FOLDER_PATH\=places.app/SystemExtensions
    export SYSTEM_EXTENSIONS_FOLDER_PATH_SHALLOW_BUNDLE_NO\=places.app/Library/SystemExtensions
    export SYSTEM_EXTENSIONS_FOLDER_PATH_SHALLOW_BUNDLE_YES\=places.app/SystemExtensions
    export SYSTEM_KEXT_INSTALL_PATH\=/System/Library/Extensions
    export SYSTEM_LIBRARY_DIR\=/System/Library
    export TAPI_DEMANGLE\=YES
    export TAPI_ENABLE_PROJECT_HEADERS\=NO
    export TAPI_LANGUAGE\=objective-c
    export TAPI_LANGUAGE_STANDARD\=compiler-default
    export TAPI_USE_SRCROOT\=YES
    export TAPI_VERIFY_MODE\=Pedantic
    export TARGETED_DEVICE_FAMILY\=1,2
    export TARGETNAME\=places_iOS
    export TARGET_BUILD_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos
    export TARGET_NAME\=places_iOS
    export TARGET_TEMP_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build
    export TEMP_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build
    export TEMP_FILES_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build
    export TEMP_FILE_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build
    export TEMP_ROOT\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex
    export TEST_FRAMEWORK_SEARCH_PATHS\=\ /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/Library/Frameworks\ /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk/Developer/Library/Frameworks
    export TEST_LIBRARY_SEARCH_PATHS\=\ /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/lib
    export TOOLCHAINS\=com.apple.dt.toolchain.XcodeDefault
    export TOOLCHAIN_DIR\=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain
    export TREAT_MISSING_BASELINES_AS_TEST_FAILURES\=NO
    export TREAT_MISSING_SCRIPT_PHASE_OUTPUTS_AS_ERRORS\=NO
    export TeamIdentifierPrefix\=K7XMFG27ZR.
    export UID\=501
    export UNINSTALLED_PRODUCTS_DIR\=/Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/UninstalledProducts
    export UNLOCALIZED_RESOURCES_FOLDER_PATH\=places.app
    export UNLOCALIZED_RESOURCES_FOLDER_PATH_SHALLOW_BUNDLE_NO\=places.app/Resources
    export UNLOCALIZED_RESOURCES_FOLDER_PATH_SHALLOW_BUNDLE_YES\=places.app
    export UNSTRIPPED_PRODUCT\=NO
    export USER\=dome
    export USER_APPS_DIR\=/Users/dome/Applications
    export USER_LIBRARY_DIR\=/Users/dome/Library
    export USE_DYNAMIC_NO_PIC\=YES
    export USE_HEADERMAP\=YES
    export USE_HEADER_SYMLINKS\=NO
    export VALIDATE_DEVELOPMENT_ASSET_PATHS\=YES_ERROR
    export VALIDATE_PRODUCT\=NO
    export VALID_ARCHS\=arm64\ \ arm64-sim
    export VERBOSE_PBXCP\=NO
    export VERSIONPLIST_PATH\=places.app/version.plist
    export VERSION_INFO_BUILDER\=dome
    export VERSION_INFO_FILE\=places_vers.c
    export VERSION_INFO_STRING\=\"@\(\#\)PROGRAM:places\ \ PROJECT:places-\"
    export WORKSPACE_DIR\=/Users/dome/work/general/tauri/places/src-tauri/gen/apple/places.xcodeproj
    export WRAPPER_EXTENSION\=app
    export WRAPPER_NAME\=places.app
    export WRAPPER_SUFFIX\=.app
    export WRAP_ASSET_PACKS_IN_SEPARATE_DIRECTORIES\=NO
    export XCODE_APP_SUPPORT_DIR\=/Applications/Xcode.app/Contents/Developer/Library/Xcode
    export XCODE_PRODUCT_BUILD_VERSION\=16C5032a
    export XCODE_VERSION_ACTUAL\=1620
    export XCODE_VERSION_MAJOR\=1600
    export XCODE_VERSION_MINOR\=1620
    export XPCSERVICES_FOLDER_PATH\=places.app/XPCServices
    export YACC\=yacc
    export _WRAPPER_CONTENTS_DIR_SHALLOW_BUNDLE_NO\=/Contents
    export _WRAPPER_PARENT_PATH_SHALLOW_BUNDLE_NO\=/..
    export _WRAPPER_RESOURCES_DIR_SHALLOW_BUNDLE_NO\=/Resources
    export __IS_NOT_MACOS\=YES
    export __IS_NOT_MACOS_macosx\=NO
    export __IS_NOT_SIMULATOR\=YES
    export __IS_NOT_SIMULATOR_simulator\=NO
    export arch\=undefined_arch
    export diagnostic_message_length\=352
    export variant\=normal
    /bin/sh -c /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Script-6079DD776A3DEEB7D06B4644.sh

> places@0.1.0 tauri
> tauri ios xcode-script -v --platform iOS --sdk-root /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS18.2.sdk --framework-search-paths /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos  "." --header-search-paths /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Products/debug-iphoneos/include  --gcc-preprocessor-definitions  DEBUG=1 --configuration debug arm64

   Compiling gemm-f16 v0.17.1
   Compiling image v0.25.5
   Compiling monostate v0.1.13
   Compiling rustix v0.38.43
   Compiling tao v0.31.1
   Compiling num-rational v0.2.4
   Compiling wry v0.48.1
   Compiling ort-sys v2.0.0-rc.9
   Compiling hf-hub v0.3.2
   Compiling rand v0.6.5
   Compiling rayon-cond v0.3.0
   Compiling swift-rs v1.0.7
   Compiling tauri-plugin-opener v2.2.4
   Compiling selectors v0.25.0
   Compiling rand_pcg v0.1.2
   Compiling markup5ever v0.12.1
   Compiling tauri-runtime v2.3.0
   Compiling tauri-codegen v2.0.4
   Compiling onig v6.4.0
   Compiling strum_macros v0.26.4
   Compiling rand_chacha v0.1.1
   Compiling md-5 v0.10.6
error: failed to run custom build command for `ort-sys v2.0.0-rc.9`

Caused by:
  process didn't exit successfully: `/Users/dome/work/general/tauri/places/src-tauri/target/debug/build/ort-sys-7a84b786ef0a2991/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=LIBONNXRUNTIME_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS_aarch64-apple-ios
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS_aarch64_apple_ios
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG_ALLOW_CROSS
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS
  cargo:rerun-if-env-changed=PKG_CONFIG_aarch64-apple-ios
  cargo:rerun-if-env-changed=PKG_CONFIG_aarch64_apple_ios
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_aarch64-apple-ios
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_aarch64_apple_ios
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  onnxruntime not found using pkg-config, falling back to manual setup.
  cargo:rerun-if-env-changed=ORT_LIB_LOCATION
  cargo:rerun-if-env-changed=ORT_LIB_PROFILE
  cargo:rerun-if-env-changed=ORT_PREFER_DYNAMIC_LINK
  cargo:rerun-if-env-changed=ORT_CXX_STDLIB
  cargo:rerun-if-env-changed=CXXSTDLIB
  selected feature set: cu12

  --- stderr

  thread 'main' panicked at /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ort-sys-2.0.0-rc.9/build.rs:441:17:
  downloaded binaries not available for target aarch64-apple-ios (note: also requested features `cu12`)
  you may have to compile ONNX Runtime from source
  stack backtrace:
     0: rust_begin_unwind
               at /rustc/419b3e2d3e350822550eee0e82eeded4d324d584/library/std/src/panicking.rs:695:5
     1: core::panicking::panic_fmt
               at /rustc/419b3e2d3e350822550eee0e82eeded4d324d584/library/core/src/panicking.rs:75:14
     2: build_script_build::prepare_libort_dir
               at ./build.rs:441:5
     3: build_script_build::real_main
               at ./build.rs:524:34
     4: build_script_build::main
               at ./build.rs:558:5
     5: core::ops::function::FnOnce::call_once
               at /Users/dome/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
warning: build failed, waiting for other jobs to finish...
error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:2024:18
     |
2024 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.h[7]",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.h[7]
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:2000:18
     |
2000 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.h[3]",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.h[3]
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:2006:18
     |
2006 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.h[4]",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.h[4]
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:2012:18
     |
2012 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.h[5]",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.h[5]
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:1994:18
     |
1994 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.h[2]",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.h[2]
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:2018:18
     |
2018 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.h[6]",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.h[6]
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:1982:18
     |
1982 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.h[0]",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.h[0]
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:1988:18
     |
1988 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.h[1]",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.h[1]
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:1954:18
     |
1954 |                 "fadd {0:v}.8h, {1:v}.8h, {2:v}.8h",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fadd v0.8h, v1.8h, v2.8h
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:1966:18
     |
1966 |                 "fmla {0:v}.8h, {1:v}.8h, {2:v}.8h",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmla v0.8h, v1.8h, v2.8h
     |     ^

error: instruction requires: fullfp16
    --> /Users/dome/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gemm-common-0.17.1/src/simd.rs:1940:18
     |
1940 |                 "fmul {0:v}.8h, {1:v}.8h, {2:v}.8h",
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: instantiated into assembly here
    --> <inline asm>:1:2
     |
1    |     fmul v0.8h, v1.8h, v2.8h
     |     ^

error: could not compile `gemm-f16` (lib) due to 11 previous errors
Failed to run `cargo build`: command ["cargo", "build", "--package", "places", "--manifest-path", "/Users/dome/work/general/tauri/places/src-tauri/Cargo.toml", "--target", "aarch64-apple-ios", "--features", "tauri/rustls-tls", "--lib", "--no-default-features"] exited with code 101
    Error Failed to run `cargo build`: command ["cargo", "build", "--package", "places", "--manifest-path", "/Users/dome/work/general/tauri/places/src-tauri/Cargo.toml", "--target", "aarch64-apple-ios", "--features", "tauri/rustls-tls", "--lib", "--no-default-features"] exited with code 101
Command PhaseScriptExecution failed with a nonzero exit code

** BUILD FAILED **


The following build commands failed:
        PhaseScriptExecution Build\ Rust\ Code /Users/dome/Library/Developer/Xcode/DerivedData/places-gfolaefvccdstqfennjxcrtukefi/Build/Intermediates.noindex/places.build/debug-iphoneos/places_iOS.build/Script-6079DD776A3DEEB7D06B4644.sh (in target 'places_iOS' from project 'places')
        Building workspace places with scheme places_iOS and configuration debug
(2 failures)
    Error command ["xcodebuild"] exited with code 65

```
