import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class RainbowBloc
    extends SpecificEffectBloc<RainbowEffectEvent, RainbowEffect> {
  RainbowBloc(RainbowEffect effect) : super(effect) {
    on<RainbowSaturationEvent>(
        (event, emit) => emit(state..saturation = event.saturation));
    on<RainbowValueEvent>((event, emit) => emit(state..value = event.value));
    on<RainbowStepEvent>((event, emit) => emit(state..step = event.step));
  }
}

abstract class RainbowEffectEvent {}

class RainbowSaturationEvent extends RainbowEffectEvent {
  double saturation;

  RainbowSaturationEvent(this.saturation);
}

class RainbowValueEvent extends RainbowEffectEvent {
  double value;

  RainbowValueEvent(this.value);
}

class RainbowStepEvent extends RainbowEffectEvent {
  double step;

  RainbowStepEvent(this.step);
}
