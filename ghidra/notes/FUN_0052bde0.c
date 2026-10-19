
undefined4 __thiscall FUN_0052bde0(void *this,int *param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  
  if (param_1 != (int *)0x0) {
    iVar1 = (**(code **)(*param_1 + 0x204))();
    iVar2 = (**(code **)(*(int *)this + 0x1ec))();
    if (((iVar1 != iVar2) || (uVar3 = 1, (*(byte *)(param_1 + 0x14) & 1) == 0)) ||
       (((*(uint *)((int)this + 0x24) ^ param_1[9]) & 0xc0) != 0)) {
      uVar3 = 0;
    }
    return uVar3;
  }
  return 0;
}

