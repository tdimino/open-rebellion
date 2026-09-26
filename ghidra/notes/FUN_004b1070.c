
int * __thiscall FUN_004b1070(void *this,undefined4 *param_1)

{
  undefined4 *puVar1;
  void *pvVar2;
  void *pvVar3;
  int iVar4;
  int *piVar5;
  int *local_30;
  void *local_2c;
  void *local_28;
  uint local_24;
  int local_20;
  uint *local_1c;
  undefined1 local_18 [4];
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638ef8;
  local_c = ExceptionList;
  local_30 = (int *)0x0;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_005f4950(&local_24,0);
  local_4 = 0;
  *param_1 = 0;
  local_1c = (uint *)((int)this + 0x30);
  local_2c = FUN_004f5940((void *)(*(int *)((int)this + 0x10) + 0x2c),local_1c);
  if (local_2c == (void *)0x0) {
    local_2c = (void *)thunk_FUN_005f5060(*(int *)((int)this + 0x10) + 0x2c);
  }
  piVar5 = (int *)0x0;
  if (local_2c != (void *)0x0) {
    do {
      if (local_20 != 0) break;
      puVar1 = FUN_00403040(local_2c,local_18);
      iVar4 = *(int *)((int)this + 0xc);
      local_10 = &stack0xffffffbc;
      local_4._0_1_ = 1;
      FUN_004f26d0(&stack0xffffffbc,puVar1);
      pvVar2 = FUN_00419e40((void *)(iVar4 + 0x78));
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      if (pvVar2 != (void *)0x0) {
        local_24 = *(uint *)((int)pvVar2 + 0x38);
        if (((local_24 & 0xf0000000) != 0) && ((local_24 & 2) == 0)) {
          if (*(int *)((int)pvVar2 + 0x28) == *(int *)((int)pvVar2 + 0x2c)) {
            iVar4 = *(int *)((int)this + 0xc);
            local_10 = &stack0xffffffbc;
            FUN_004f26d0(&stack0xffffffbc,(undefined4 *)((int)pvVar2 + 0x2c));
            pvVar3 = FUN_00403d30((void *)(iVar4 + 0x2c));
            if (pvVar3 != (void *)0x0) {
              if ((local_24 & 0x10000000) == 0) {
                if ((local_24 & 0x20000000) == 0) {
                  if ((local_24 & 0x40000000) != 0) {
                    if (((local_24 & 0x10) == 0) || (*(int *)((int)pvVar3 + 0x110) < 1)) {
                      iVar4 = FUN_004ae320(this,(int)pvVar2,pvVar3,&local_28);
                      if (iVar4 != 0) {
                        local_30 = FUN_004b1330(this,pvVar2,local_28);
                      }
                    }
                    else {
                      local_30 = FUN_004b1330(this,pvVar2,(void *)0x223);
                    }
                    goto LAB_004b1186;
                  }
                  if ((local_24 & 0x80000000) != 0) {
                    iVar4 = FUN_004ae440((int)pvVar2,(int)pvVar3);
                    if (iVar4 == 0) goto LAB_004b1186;
                    local_30 = FUN_004b1440(this,(int)pvVar2);
                  }
                }
                else if (((local_24 & 0x10) == 0) || (*(int *)((int)pvVar3 + 0x110) < 1)) {
                  iVar4 = FUN_004ae240(this,(int)pvVar2,pvVar3);
                  if (iVar4 == 0) {
                    iVar4 = FUN_004ae320(this,(int)pvVar2,pvVar3,&local_28);
                    if (iVar4 == 0) goto LAB_004b1186;
                    local_30 = FUN_004b1330(this,pvVar2,local_28);
                  }
                  else {
                    local_30 = FUN_004b1330(this,pvVar2,(void *)0x234);
                  }
                }
                else {
                  local_30 = FUN_004b1330(this,pvVar2,(void *)0x223);
                }
              }
              else {
                iVar4 = FUN_004ae150((int)pvVar2,(int)pvVar3);
                if (iVar4 == 0) goto LAB_004b1186;
              }
              goto LAB_004b1198;
            }
          }
LAB_004b1186:
          *(uint *)((int)pvVar2 + 0x38) = *(uint *)((int)pvVar2 + 0x38) & 0xfffffff;
          FUN_004ece80((undefined4 *)((int)pvVar2 + 0x2c));
        }
      }
LAB_004b1198:
      local_2c = *(void **)((int)local_2c + 0x10);
      if (local_30 != (int *)0x0) {
        local_20 = 1;
        if (local_2c == (void *)0x0) break;
        puVar1 = FUN_00403040(local_2c,local_14);
        local_4._0_1_ = 2;
        FUN_004f26d0(local_1c,puVar1);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
      }
    } while (local_2c != (void *)0x0);
    piVar5 = local_30;
    if (local_2c != (void *)0x0) goto LAB_004b1203;
  }
  *param_1 = 1;
  FUN_004ece80(local_1c);
LAB_004b1203:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return piVar5;
}

