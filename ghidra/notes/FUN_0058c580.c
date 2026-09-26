
uint __thiscall FUN_0058c580(void *this,void *param_1)

{
  int iVar1;
  void *pvVar2;
  uint *puVar3;
  int iVar4;
  uint uVar5;
  int *piVar6;
  bool bVar7;
  int local_70;
  uint local_6c;
  int local_68;
  uint local_64;
  int local_60;
  int local_5c;
  int local_58;
  uint local_54;
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064ff40;
  pvStack_c = ExceptionList;
  local_68 = 0;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_0055eb80(*(int *)((int)this + 0x2c),&local_68);
  uVar5 = (uint)(iVar1 != 0);
  FUN_00539d70(local_4c,*(undefined4 *)((int)this + 0x10),1);
  local_4 = 0;
  FUN_00526cf0(local_2c,*(undefined4 *)((int)this + 0x10),1);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_6c);
  local_4._0_1_ = 2;
  FUN_00513120((int)local_2c);
  if (local_10 != 0) {
    puVar3 = &local_64;
    pvVar2 = (void *)FUN_0052bed0((int)local_2c);
    puVar3 = FUN_004025b0(pvVar2,puVar3);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_6c,puVar3);
    local_4._0_1_ = 2;
    FUN_00619730();
  }
  local_64 = 0;
  local_70 = 0;
  while (((uVar5 != 0 && (local_70 < local_68)) && (local_64 == 0))) {
    iVar1 = FUN_00513180((int)local_4c);
    iVar4 = FUN_00513180((int)local_2c);
    uVar5 = FUN_0055ebc0(iVar1,iVar4,*(int *)(*(int *)((int)this + 0x10) + 0x5c),
                         *(int *)(*(int *)((int)this + 0x10) + 100),&local_58,&local_50,&local_60,
                         &local_5c);
    if (uVar5 != 0) {
      if (local_60 == 0) {
        if (local_5c == 0) {
          if (local_58 == 0) {
            local_64 = 1;
          }
          else {
            piVar6 = (int *)0x0;
            iVar1 = 0;
            FUN_00513120((int)local_4c);
            while ((local_30 != 0 && (piVar6 == (int *)0x0))) {
              puVar3 = &local_54;
              pvVar2 = (void *)FUN_0052bed0((int)local_4c);
              puVar3 = FUN_004025b0(pvVar2,puVar3);
              bVar7 = local_6c != *puVar3;
              FUN_00619730();
              if (bVar7) {
                if (iVar1 == local_50) {
                  piVar6 = (int *)FUN_0052bed0((int)local_4c);
                }
                else {
                  iVar1 = iVar1 + 1;
                }
              }
              FUN_005130d0((int)local_4c);
            }
            uVar5 = 0;
            if (piVar6 != (int *)0x0) {
              uVar5 = (**(code **)(*piVar6 + 0xac))(0xe,param_1);
            }
          }
        }
        else {
          uVar5 = FUN_00509dc0(*(void **)((int)this + 0x10),
                               *(int *)((int)*(void **)((int)this + 0x10) + 100) + -1,param_1);
        }
      }
      else {
        uVar5 = FUN_00509cc0(*(void **)((int)this + 0x10),
                             *(int *)((int)*(void **)((int)this + 0x10) + 0x5c) + -1,param_1);
      }
    }
    local_70 = local_70 + 1;
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00526df0(local_2c);
  local_4 = 0xffffffff;
  FUN_00539e70(local_4c);
  ExceptionList = pvStack_c;
  return uVar5;
}

