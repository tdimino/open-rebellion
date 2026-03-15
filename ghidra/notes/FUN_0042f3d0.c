
uint __fastcall FUN_0042f3d0(void *param_1)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  uint uVar4;
  void *pvVar5;
  uint local_10;
  
  bVar1 = false;
  local_10 = 0;
  for (iVar3 = thunk_FUN_005f5060((int)param_1); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
    if (bVar1) goto LAB_0042f485;
    uVar4 = *(uint *)(iVar3 + 0x34) >> 0x18;
    if ((uVar4 < 0x80) || (0x8f < uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      local_10 = *(uint *)(iVar3 + 0x18);
      bVar1 = true;
    }
    if (((!bVar1) &&
        (pvVar5 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x10) + 0x44),(uint *)(iVar3 + 0x34)
                              ), pvVar5 != (void *)0x0)) && (0 < *(int *)((int)pvVar5 + 0x18c))) {
      local_10 = *(uint *)(iVar3 + 0x18);
      bVar1 = true;
    }
  }
  if (bVar1) {
LAB_0042f485:
    pvVar5 = (void *)FUN_005f5500(param_1,local_10);
    if (pvVar5 != (void *)0x0) {
      thunk_FUN_00477190(pvVar5);
      FUN_005f54a0(param_1,local_10);
      return local_10;
    }
  }
  return local_10;
}

