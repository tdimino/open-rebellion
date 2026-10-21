
void __thiscall FUN_005588f0(void *this,uint param_1)

{
  *(uint *)this = (param_1 ^ *(uint *)this) & 3 ^ *(uint *)this;
  return;
}

