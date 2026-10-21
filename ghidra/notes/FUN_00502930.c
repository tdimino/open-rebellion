
undefined4 __thiscall FUN_00502930(void *this,undefined4 param_1,int param_2,undefined4 param_3)

{
  int iVar1;
  int iVar2;
  
  iVar1 = (**(code **)(*(int *)this + 0x248))();
  iVar2 = (**(code **)(*(int *)this + 0x298))(param_3);
  if ((iVar2 != 0) && (param_2 <= iVar1)) {
    return 1;
  }
  return 0;
}

