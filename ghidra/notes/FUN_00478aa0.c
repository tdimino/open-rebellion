
int * __fastcall FUN_00478aa0(void *param_1)

{
  int iVar1;
  bool bVar2;
  ushort uVar3;
  void *this;
  uint *puVar4;
  void *pvVar5;
  int iVar6;
  int *piVar7;
  undefined4 *puVar8;
  void *local_40;
  uint local_3c;
  undefined4 local_38;
  void *local_34;
  int *local_30;
  undefined1 local_2c [4];
  undefined1 local_28 [4];
  int local_24;
  undefined1 local_20 [4];
  undefined1 local_1c [4];
  void *local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006332f2;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_3c = 0;
  local_30 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_34 = param_1;
  FUN_004ece30(&local_38);
  local_4 = 0;
  FUN_004f4340(local_14);
  local_24 = (int)param_1 + 0x24;
  local_4._0_1_ = 1;
  this = (void *)thunk_FUN_005f5060(local_24);
  do {
    while( true ) {
      local_40 = this;
      if ((this == (void *)0x0) || (bVar2)) {
        iVar6 = thunk_FUN_005f5060((int)local_14);
        if (iVar6 != 0) {
          FUN_004ece30(&local_3c);
          local_4._0_1_ = 6;
          puVar8 = FUN_00479220(param_1,local_1c,&local_38);
          local_4._0_1_ = 7;
          FUN_004f26d0(&local_3c,puVar8);
          local_4 = CONCAT31(local_4._1_3_,6);
          FUN_00619730();
          piVar7 = (int *)FUN_004f5cd0(0x270);
          local_30 = piVar7;
          if (piVar7 != (int *)0x0) {
            piVar7[8] = *(int *)((int)param_1 + 0x20);
            (**(code **)(*piVar7 + 0x24))(local_14);
            (**(code **)(*piVar7 + 0x2c))(&local_40);
          }
          local_4._0_1_ = 1;
          FUN_00619730();
        }
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_004f4380(local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        ExceptionList = pvStack_c;
        return local_30;
      }
      puVar4 = FUN_00403040(this,local_2c);
      local_4._0_1_ = 2;
      pvVar5 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x58) + 0x58),puVar4);
      local_4._0_1_ = 1;
      FUN_00619730();
      bVar2 = false;
      if (((pvVar5 == (void *)0x0) || ((*(uint *)((int)pvVar5 + 0x24) & 0x402800) != 0)) ||
         (iVar6 = FUN_00484510((int)pvVar5), iVar6 != 0)) break;
      bVar2 = false;
      if ((*(int *)((int)pvVar5 + 0x7c) <= *(int *)((int)param_1 + 0x6c)) &&
         (bVar2 = *(int *)((int)pvVar5 + 0x80) <= *(int *)((int)param_1 + 0x70), bVar2)) {
        if (((*(byte *)((int)pvVar5 + 0x24) & 2) != 0) && (*(int *)((int)param_1 + 0x90) < 1)) {
          bVar2 = false;
        }
        if (bVar2) {
          if (((*(byte *)((int)pvVar5 + 0x24) & 0xc) != 0) &&
             (*(int *)((int)param_1 + 0x74) < *(int *)((int)pvVar5 + 0x6c))) {
            bVar2 = false;
          }
          if (bVar2) {
            if ((0 < *(int *)((int)pvVar5 + 0x60)) &&
               (*(int *)((int)param_1 + 0x78) < *(int *)((int)pvVar5 + 0x60))) {
              bVar2 = false;
            }
            if (bVar2) {
              if ((0 < *(int *)((int)pvVar5 + 0x48)) &&
                 (*(int *)((int)param_1 + 0x80) < *(int *)((int)pvVar5 + 0x48))) {
                bVar2 = false;
              }
              if (bVar2) {
                if ((*(int *)((int)param_1 + 0x7c) < 1) &&
                   ((*(byte *)((int)pvVar5 + 0x24) & 0x80) != 0)) {
                  bVar2 = false;
                }
                if (bVar2) {
                  if ((*(int *)((int)param_1 + 0x84) < 1) &&
                     ((*(uint *)((int)pvVar5 + 0x24) & 0x400) != 0)) {
                    bVar2 = false;
                  }
                  if (bVar2) {
                    piVar7 = FUN_00403040(pvVar5,local_28);
                    iVar6 = *piVar7;
                    iVar1 = *(int *)((int)param_1 + 0x38);
                    FUN_00619730();
                    param_1 = local_34;
                    if ((iVar6 == iVar1) &&
                       (uVar3 = FUN_005f50e0(local_24), param_1 = local_34, 1 < uVar3)) {
                      bVar2 = false;
                      *(uint *)((int)pvVar5 + 0x24) = *(uint *)((int)pvVar5 + 0x24) | 0x2000000;
                    }
                    break;
                  }
                }
              }
            }
          }
        }
      }
LAB_00478d2b:
      this = *(void **)((int)this + 0x10);
    }
    if (!bVar2) goto LAB_00478d2b;
    this = *(void **)((int)this + 0x10);
    FUN_004f26d0(&local_38,(undefined4 *)((int)pvVar5 + 0x2c));
    pvVar5 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 3;
    local_18 = pvVar5;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      puVar8 = FUN_00403040(local_40,local_20);
      local_3c = local_3c | 1;
      local_4 = CONCAT31(local_4._1_3_,4);
      pvVar5 = FUN_004f5b10(pvVar5,puVar8,0);
    }
    local_4 = 3;
    if ((local_3c & 1) != 0) {
      local_3c = local_3c & 0xfffffffe;
      FUN_00619730();
    }
    local_4._0_1_ = 1;
    if (pvVar5 != (void *)0x0) {
      FUN_004f57b0(local_14,pvVar5);
      puVar4 = FUN_00403040(local_40,local_1c);
      local_4._0_1_ = 5;
      FUN_004790d0(param_1,puVar4);
      local_4._0_1_ = 1;
      FUN_00619730();
    }
  } while( true );
}

