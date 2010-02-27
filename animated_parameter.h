#pragma once

template <typename type_t>
class animated_parameter_t
{
public:
    enum LoopType { None, ForwardLoop, PingPongLoop };
    enum Direction { Forward, Backward };

private:
    type_t value_;
    type_t min, max; // if min == max -> no constraint
    type_t speed;
    LoopType loop_type;
    Direction direction;

public:
    animated_parameter_t(type_t start_value, type_t speed = 1, type_t min = 0, type_t max = 0, LoopType loop = None, Direction start_dir = Forward);

    /* For elapsed time in milliseconds and speed, calculate value change, set value_ to new result and return it. */
    type_t animate(int elapsedTimeMs);
    inline type_t value() { return value_; }
};

template <typename type_t>
animated_parameter_t<type_t>::animated_parameter_t(type_t start_value, type_t speed, type_t min, type_t max, LoopType loop, Direction start_dir)
    : value_(start_value)
    , min(min)
    , max(max)
    , speed(speed)
    , loop_type(loop)
    , direction(start_dir)
{
}

template <typename type_t>
type_t animated_parameter_t<type_t>::animate(int elapsedTimeMs)
{
    value_ += (speed / 1000) * elapsedTimeMs * (direction == Forward ? 1 : -1);

    if (min != max)
    {
        if (value_ > max)
        {
            if (loop_type == PingPongLoop)
            {
                value_ = max;
                direction = (direction == Forward ? Backward : Forward);
            }
            else if (loop_type == ForwardLoop)
            {
                value_ = min;
            }
            else
                value_ = max;
        }
        if (value_ < min)
        {
            if (loop_type == PingPongLoop)
            {
                value_ = min;
                direction = (direction == Forward ? Backward : Forward);
            }
            else if (loop_type == ForwardLoop)
            {
                value_ = max;
            }
            else
                value_ = min;
        }
    }

    return value_;
}
