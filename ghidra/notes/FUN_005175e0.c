
int __cdecl FUN_005175e0(void *param_1,void *param_2)

{
  uint uVar1;
  uint uVar2;
  int iVar3;
  void *this;
  int iVar4;
  uint *puVar5;
  void *pvVar6;
  uint *puVar7;
  void *this_00;
  undefined1 *puVar8;
  int local_44;
  undefined1 local_40 [4];
  uint local_3c;
  uint local_38;
  uint uStack_34;
  undefined4 uStack_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642920;
  local_c = ExceptionList;
  local_44 = 1;
  ExceptionList = &local_c;
  this = (void *)FUN_00505ef0();
  iVar3 = local_44;
  while (this != (void *)0x0) {
    FUN_0052c0d0(local_2c,this);
    local_4._0_1_ = 1;
    local_4._1_3_ = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar4 = FUN_0052bed0((int)local_2c);
      local_44 = iVar3;
      if (*(int *)(iVar4 + 0x58) == 0) {
        puVar5 = FUN_004025b0(param_1,&local_3c);
        puVar8 = local_40;
        local_4._0_1_ = 2;
        pvVar6 = (void *)FUN_0052bed0((int)local_2c);
        puVar7 = FUN_004c27b0(pvVar6,puVar8);
        uVar1 = *puVar7;
        uVar2 = *puVar5;
        FUN_00619730();
        local_4._0_1_ = 1;
        FUN_00619730();
        if (uVar1 == uVar2) {
          puVar5 = FUN_004025b0(this,&local_38);
          local_4._0_1_ = 3;
          pvVar6 = param_2;
          this_00 = (void *)FUN_0052bed0((int)local_2c);
          iVar4 = FUN_005298d0(this_00,(int *)puVar5,pvVar6);
          if ((iVar4 == 0) || (local_44 = 1, iVar3 == 0)) {
            local_44 = 0;
          }
          local_4._0_1_ = 1;
          FUN_00619730();
        }
      }
      FUN_005130d0((int)local_2c);
      iVar3 = local_44;
    }
    local_4 = 0xffffffff;
    FUN_0052c1f0(local_2c);
    uStack_34 = 0x90;
    uStack_30 = 0x98;
    local_4 = 0;
    this = (void *)FUN_004f6010(this,&uStack_34,1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar3;
}

