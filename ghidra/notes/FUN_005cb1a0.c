
void __thiscall FUN_005cb1a0(void *this,int param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  int iVar3;
  void *pvVar4;
  int local_1c;
  int local_18;
  int local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065429c;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(undefined4 **)((int)this + 0x54) != (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)**(undefined4 **)((int)this + 0x54))();
    *(undefined4 *)((int)this + 0x54) = 0;
  }
  local_1c = 0;
  local_18 = 0;
  local_14 = 0;
  local_10 = (void *)0xbf800000;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)FUN_00618b70(0x48);
    uStack_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar1);
    }
    uStack_4 = 0xffffffff;
  }
  piVar2 = (int *)FUN_005a6340(DAT_006bc470,*(uint *)((int)this + 0x6c));
  if (piVar2 != (int *)0x0) {
    local_1c = piVar2[0xc];
    local_18 = piVar2[0xd];
    local_14 = piVar2[0xe];
    local_10 = (void *)piVar2[0xf];
  }
  if (param_1 == 1) {
    *(undefined4 *)((int)this + 0x58) = 1;
    pvVar4 = (void *)FUN_00618b70(0x140);
    uStack_4 = 1;
    if (pvVar4 != (void *)0x0) {
      piVar2 = FUN_005eb860(pvVar4,*(uint *)((int)this + 0x6c),local_1c,local_18,local_14);
      goto LAB_005cb374;
    }
  }
  else if ((param_1 == 3) && (iVar3 = (**(code **)(*piVar2 + 0x20))(), iVar3 == 1)) {
    *(undefined4 *)((int)this + 0x58) = 3;
    pvVar4 = (void *)FUN_00618b70(0x144);
    uStack_4 = 2;
    if (pvVar4 != (void *)0x0) {
      piVar2 = FUN_005c72b0(pvVar4,*(uint *)((int)this + 0x6c));
      goto LAB_005cb374;
    }
  }
  else {
    *(undefined4 *)((int)this + 0x58) = 2;
    pvVar4 = (void *)FUN_00618b70(0x140);
    uStack_4 = 3;
    if (pvVar4 != (void *)0x0) {
      piVar2 = FUN_005ec310(pvVar4,*(uint *)((int)this + 0x6c),local_1c,local_18,local_14,local_10);
      goto LAB_005cb374;
    }
  }
  piVar2 = (int *)0x0;
LAB_005cb374:
  *(int **)((int)this + 0x54) = piVar2;
  uStack_4 = 0xffffffff;
  (**(code **)(*piVar2 + 0x1c))();
  for (piVar2 = (int *)(**(code **)(*(int *)((int)this + 0x1c) + 0x10))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
    iVar3 = (**(code **)(*piVar2 + 0x10))();
    if (*(int *)(iVar3 + 0xec) != *(int *)((int)this + 0x6c)) {
      (**(code **)(**(int **)((int)this + 0x54) + 0x1c))();
    }
  }
  ExceptionList = local_10;
  return;
}

