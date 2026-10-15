
uint __thiscall FUN_006075b0(void *this,int *param_1,undefined4 param_2)

{
  uint uVar1;
  
  uVar1 = 0;
  if (param_1 != (int *)0x0) {
    *(int **)((int)this + 0xa4) = param_1;
    FUN_006076e0((int)this);
    uVar1 = FUN_00607610(this,param_1,param_2);
  }
  return uVar1;
}

