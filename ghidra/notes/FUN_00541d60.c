
undefined4 __fastcall FUN_00541d60(int param_1)

{
  uint *puVar1;
  bool bVar2;
  int iVar3;
  void *pvVar4;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646f78;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_28,param_1 + 0x20);
  puVar1 = (uint *)(param_1 + 0x50);
  bVar2 = false;
  local_4 = 0;
  iVar3 = FUN_004ece60(puVar1);
  if (iVar3 != 0) {
    if ((0x3f < *puVar1 >> 0x18) && (*puVar1 >> 0x18 < 0x80)) {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      pvVar4 = (void *)FUN_00505970(puVar1);
      if (pvVar4 != (void *)0x0) {
        FUN_004f8040(pvVar4,local_28);
      }
    }
  }
  puVar1 = (uint *)(param_1 + 0x4c);
  iVar3 = FUN_004ece60(puVar1);
  if (iVar3 != 0) {
    if ((*puVar1 >> 0x18 < 0x40) || (0x7f < *puVar1 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      pvVar4 = (void *)FUN_00505970(puVar1);
      if (pvVar4 != (void *)0x0) {
        FUN_004f8090(pvVar4,local_28);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return 0;
}

