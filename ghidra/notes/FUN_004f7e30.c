
int __thiscall FUN_004f7e30(void *this,uint *param_1,void *param_2)

{
  int iVar1;
  void *this_00;
  int *piVar2;
  int *piVar3;
  int iVar4;
  bool bVar5;
  undefined4 local_4;
  
  local_4 = 1;
  this_00 = (void *)FUN_00539fd0(this,0);
  if (this_00 != (void *)0x0) {
    for (piVar2 = FUN_00539f80(this_00,param_1); piVar2 != (int *)0x0;
        piVar2 = FUN_00539fa0(piVar2,param_1)) {
      FUN_004f9510(piVar2,*(uint *)((int)this + 0x24) >> 4 & 3,param_2);
    }
  }
  piVar2 = FUN_00539f80(this,param_1);
  iVar1 = local_4;
  while (piVar2 != (int *)0x0) {
    piVar3 = FUN_00539fa0(piVar2,param_1);
    iVar4 = FUN_004f7f20(piVar2,param_2);
    piVar2 = piVar3;
    if ((iVar4 == 0) || (local_4 = 1, bVar5 = iVar1 == 0, iVar1 = local_4, bVar5)) {
      local_4 = 0;
      iVar1 = local_4;
    }
  }
  return iVar1;
}

