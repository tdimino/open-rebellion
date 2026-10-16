
void __thiscall FUN_004f7b80(void *param_1,void *param_2)

{
  uint uVar1;
  
  uVar1 = *(uint *)((int)param_1 + 0x50);
  if ((((uVar1 & 0x40) != 0) && ((uVar1 & 4) != 0)) && ((uVar1 & 0x10) == 0)) {
    FUN_004f7410(param_1,1,param_2);
    return;
  }
  FUN_004f7410(param_1,0,param_2);
  return;
}

