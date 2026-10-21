
void __thiscall FUN_00558930(void *this,uint param_1)

{
  *(uint *)this = *(uint *)this & 0xffffffcf | (param_1 & 3) << 4;
  return;
}

