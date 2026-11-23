OUT_RTC_TIMER := rtc_timer

build: build_pkg_rtc_timer
clean: rtc_timer_clean

build_pkg_rtc_timer: 
	cp target/x86_64-unknown-linux-gnu/debug/librtc_timer.so $(RTC_TIMER_ROOT)/pkg/lib.so
	cd $(RTC_TIMER_ROOT)/pkg && tar -cf rtc_timer.tar ./*
	mv $(RTC_TIMER_ROOT)/pkg/rtc_timer.tar ./pkgs

rtc_timer_clean:
	rm -f $(RTC_TIMER_ROOT)/$(OUT_RTC_TIMER).so
	rm -f $(RTC_TIMER_ROOT)/$(OUT_RTC_TIMER).aot
	rm -f $(RTC_TIMER_ROOT)/pkg/lib.so
	rm -f $(RTC_TIMER_ROOT)/pkg/$(OUT_RTC_TIMER).tar
	rm -f pkgs/rtc_timer.tar

