
undefined4 * __thiscall
FUN_00403750(void *this,undefined4 *param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            undefined4 param_6)

{
  bool bVar1;
  undefined4 *puVar2;
  void *pvVar3;
  void *pvVar4;
  int iVar5;
  undefined1 *this_00;
  uint local_3c;
  uint local_38;
  undefined1 local_34 [4];
  undefined1 local_30 [4];
  undefined4 *local_2c;
  undefined1 *local_28;
  short local_24;
  short sStack_22;
  short local_1c;
  short sStack_1a;
  undefined2 local_18;
  undefined2 uStack_16;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062956d;
  local_c = ExceptionList;
  local_38 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_3c);
  local_4 = 0;
  puVar2 = FUN_00403c20(this,&param_1,param_1);
  local_4._0_1_ = 1;
  FUN_004f26d0(&local_3c,puVar2);
  local_4._0_1_ = 0;
  FUN_00619730();
  local_1c = 0x90;
  sStack_1a = 0;
  local_18 = 0x98;
  uStack_16 = 0;
  if ((local_3c >> 0x18 < 0x90) || (0x97 < local_3c >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar5 = *(int *)((int)this + 0x10);
    param_1 = (undefined4 *)&stack0xffffffb0;
    FUN_004f26d0(&stack0xffffffb0,&local_3c);
    pvVar3 = FUN_00403d30((void *)(iVar5 + 0x2c));
    if (pvVar3 != (void *)0x0) {
      local_24 = (short)*(undefined4 *)((int)pvVar3 + 0x34);
      sStack_22 = (short)((uint)*(undefined4 *)((int)pvVar3 + 0x34) >> 0x10);
      param_1 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (param_1 == (undefined4 *)0x0) {
        param_1 = (undefined4 *)0x0;
      }
      else {
        param_1 = FUN_0041be80(param_1,param_6);
      }
      puVar2 = param_1;
      local_4._0_1_ = 0;
      local_2c = param_1;
      if (param_1 != (undefined4 *)0x0) {
        pvVar3 = (void *)thunk_FUN_005f5060((int)this);
        for (; pvVar3 != (void *)0x0; pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
          if (((((param_2 & *(uint *)((int)pvVar3 + 0x30)) == param_2) &&
               ((param_4 & *(uint *)((int)pvVar3 + 0x30)) == 0)) &&
              ((param_3 & *(uint *)((int)pvVar3 + 0x34)) == param_3)) &&
             ((param_5 & *(uint *)((int)pvVar3 + 0x34)) == 0)) {
            puVar2 = FUN_00403040(pvVar3,local_34);
            local_4._0_1_ = 3;
            puVar2 = FUN_00403c20(this,&param_6,puVar2);
            local_4._0_1_ = 4;
            FUN_004f26d0(&local_3c,puVar2);
            local_4._0_1_ = 3;
            FUN_00619730();
            local_4._0_1_ = 0;
            FUN_00619730();
            local_14 = 0x90;
            local_10 = 0x98;
            if ((local_3c >> 0x18 < 0x90) || (0x97 < local_3c >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (bVar1) {
              iVar5 = *(int *)((int)this + 0x10);
              local_28 = &stack0xffffffb0;
              FUN_004f26d0(&stack0xffffffb0,&local_3c);
              pvVar4 = FUN_00403d30((void *)(iVar5 + 0x2c));
              if (pvVar4 != (void *)0x0) {
                local_1c = (short)*(undefined4 *)((int)pvVar4 + 0x34);
                sStack_1a = (short)((uint)*(undefined4 *)((int)pvVar4 + 0x34) >> 0x10);
                iVar5 = FUN_0041b7b0(local_24,sStack_22,local_1c,sStack_1a);
                this_00 = (undefined1 *)FUN_00618b70(0x18);
                local_4._0_1_ = 5;
                local_28 = this_00;
                if (this_00 == (undefined1 *)0x0) {
                  pvVar4 = (void *)0x0;
                }
                else {
                  puVar2 = FUN_00403040(pvVar3,local_30);
                  local_38 = local_38 | 1;
                  local_4 = CONCAT31(local_4._1_3_,6);
                  pvVar4 = FUN_0041c360(this_00,puVar2,iVar5);
                }
                local_4 = 5;
                if ((local_38 & 1) != 0) {
                  local_38 = local_38 & 0xfffffffe;
                  FUN_00619730();
                }
                local_4._0_1_ = 0;
                FUN_0041c070(param_1,pvVar4);
              }
            }
          }
          puVar2 = param_1;
        }
        FUN_0041c230((int)puVar2);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_2c;
}

