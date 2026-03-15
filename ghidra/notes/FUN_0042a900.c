
void __thiscall FUN_0042a900(void *this,uint *param_1,int *param_2)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  void *pvVar5;
  int iVar6;
  int *local_30;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  piVar4 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c5a0;
  local_c = ExceptionList;
  local_30 = param_2;
  ExceptionList = &local_c;
  iVar2 = thunk_FUN_005f5060((int)this + 0x478);
  FUN_0042b330(this,param_1,piVar4,1);
  if (piVar4 == (int *)0x0) {
    local_30 = FUN_004f2d10(*(int *)((int)this + 0x9c),param_1);
  }
  if (local_30 == (int *)0x0) {
    ExceptionList = local_c;
    return;
  }
  if ((void *)local_30[7] == (void *)0x0) {
    ExceptionList = local_c;
    return;
  }
  FUN_004025b0((void *)local_30[7],(uint *)&param_2);
  uVar3 = (uint)param_2 >> 0x18;
  local_4 = 1;
  if ((uVar3 < 0x90) || (0x97 < uVar3)) {
    if ((uVar3 < 8) || (0xf < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      if (((uint)param_2 >> 0x18 < 0x14) || (0x1b < (uint)param_2 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        if (((uint)param_2 >> 0x18 < 0x40) || (0x7f < (uint)param_2 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) {
          bVar1 = false;
          goto LAB_0042aa5b;
        }
      }
    }
  }
  bVar1 = true;
LAB_0042aa5b:
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  if ((bVar1) && (iVar2 != 0)) {
    do {
      piVar4 = (int *)FUN_00604500((void *)((int)this + 0x6c),*(uint *)(iVar2 + 0x18));
      if ((piVar4 != (int *)0x0) &&
         (pvVar5 = FUN_004f5940(piVar4 + 0x45,(uint *)&param_2), pvVar5 != (void *)0x0)) {
        (**(code **)(*piVar4 + 0x5c))(param_1,local_30);
        iVar6 = FUN_004291d0(this,piVar4[9]);
        if (iVar6 != 0) {
          FUN_00428d70(this,piVar4);
        }
      }
      iVar2 = *(int *)(iVar2 + 0x10);
    } while (iVar2 != 0);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

