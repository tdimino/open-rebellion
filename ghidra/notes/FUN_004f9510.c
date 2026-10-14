
undefined4 __thiscall FUN_004f9510(void *this,int param_1,undefined4 param_2)

{
  int *piVar1;
  undefined4 uVar2;
  
  uVar2 = 1;
  piVar1 = (int *)FUN_00539fd0(this,param_1);
  if (piVar1 != (int *)0x0) {
    uVar2 = (**(code **)(*piVar1 + 0x24))(param_2);
  }
  return uVar2;
}

