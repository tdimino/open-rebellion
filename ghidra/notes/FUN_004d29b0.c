
int * __fastcall FUN_004d29b0(void *param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint local_1c;
  int *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c170;
  local_c = ExceptionList;
  local_18 = (int *)0x0;
  ExceptionList = &local_c;
  FUN_005f4950(&local_1c,0);
  local_4 = 0;
  iVar2 = FUN_005f4960((undefined4 *)((int)param_1 + 0x1c));
  bVar1 = true;
  if (iVar2 == 1) {
    if (*(int *)((int)param_1 + 0x38) < 10) {
      pvVar4 = *(void **)((int)param_1 + 0x3c);
      if (pvVar4 == (void *)0x0) {
        uVar3 = *(uint *)((int)param_1 + 0x40) >> 0x18;
        local_14 = 0x14;
        local_10 = 0x1c;
        if ((uVar3 < 0x14) || (0x1b < uVar3)) {
          bVar1 = false;
        }
        FUN_00619730();
        if (bVar1) {
          pvVar4 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x2c) + 0x58),
                                (uint *)((int)param_1 + 0x40));
          *(void **)((int)param_1 + 0x3c) = pvVar4;
        }
        if (*(int *)((int)param_1 + 0x3c) == 0) {
          iVar2 = thunk_FUN_005f5060(*(int *)((int)param_1 + 0x2c) + 0x58);
          *(int *)((int)param_1 + 0x3c) = iVar2;
        }
      }
      else {
        local_1c = *(uint *)((int)pvVar4 + 0x24);
        if (((local_1c & 0x4000) != 0) && ((local_1c & 0x8002800) == 0)) {
          local_18 = FUN_004d2b30(param_1,pvVar4);
          *(int *)((int)param_1 + 0x38) = *(int *)((int)param_1 + 0x38) + 1;
        }
        iVar2 = *(int *)(*(int *)((int)param_1 + 0x3c) + 0x10);
        *(int *)((int)param_1 + 0x3c) = iVar2;
        if (iVar2 == 0) {
          *(undefined4 *)((int)param_1 + 0x38) = 0;
          *(undefined4 *)((int)param_1 + 0x20) = 1;
          FUN_004ece80((undefined4 *)((int)param_1 + 0x40));
        }
      }
      goto LAB_004d2ae2;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    puVar5 = FUN_00403040(*(void **)((int)param_1 + 0x3c),&local_14);
    local_4._0_1_ = 1;
    FUN_004f26d0((void *)((int)param_1 + 0x40),puVar5);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  else {
    *(undefined4 *)((int)param_1 + 0x38) = 0;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
LAB_004d2ae2:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_18;
}

