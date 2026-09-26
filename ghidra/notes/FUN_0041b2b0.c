
void * __thiscall
FUN_0041b2b0(void *this,void *param_1,uint *param_2,uint param_3,uint param_4,uint param_5,
            uint param_6,uint param_7,uint param_8)

{
  void *pvVar1;
  undefined4 *puVar2;
  int *piVar3;
  void *pvVar4;
  bool bVar5;
  uint local_54;
  int local_50;
  undefined4 local_4c;
  int local_48;
  undefined1 local_44 [4];
  undefined1 local_40 [4];
  int local_3c;
  undefined4 local_38;
  short local_34;
  short sStack_32;
  short local_2c;
  short sStack_2a;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b5bf;
  local_c = ExceptionList;
  local_38 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_4c);
  local_4 = 1;
  FUN_004ece30(&local_50);
  local_4._0_1_ = 2;
  FUN_004ece30(&local_54);
  local_4._0_1_ = 3;
  local_48 = 2000000000;
  FUN_004355d0(local_1c);
  local_4._0_1_ = 4;
  pvVar1 = FUN_004f5940((void *)((int)this + 0x44),param_2);
  if (pvVar1 != (void *)0x0) {
    puVar2 = FUN_00403040(pvVar1,&param_2);
    local_4._0_1_ = 5;
    FUN_004f26d0(&local_50,puVar2);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_34 = (short)*(undefined4 *)((int)pvVar1 + 0x40);
    sStack_32 = (short)((uint)*(undefined4 *)((int)pvVar1 + 0x40) >> 0x10);
    for (pvVar1 = (void *)thunk_FUN_005f5060((int)this + 0x44); pvVar1 != (void *)0x0;
        pvVar1 = *(void **)((int)pvVar1 + 0x10)) {
      piVar3 = FUN_00403040(pvVar1,&param_2);
      bVar5 = *piVar3 != local_50;
      FUN_00619730();
      if (bVar5) {
        local_2c = (short)*(undefined4 *)((int)pvVar1 + 0x40);
        sStack_2a = (short)((uint)*(undefined4 *)((int)pvVar1 + 0x40) >> 0x10);
        local_3c = FUN_0041b7b0(local_34,sStack_32,local_2c,sStack_2a);
        if (local_3c < local_48) {
          puVar2 = FUN_00403040(pvVar1,local_44);
          local_4._0_1_ = 6;
          pvVar4 = FUN_0041a460(this,puVar2,param_3,param_4,param_5,param_6,param_7,param_8,
                                (void *)0x1);
          local_4._0_1_ = 4;
          FUN_00619730();
          FUN_00435790(local_1c,(int)pvVar4);
          puVar2 = FUN_004357b0(local_1c,local_40);
          local_4._0_1_ = 7;
          FUN_004f26d0(&local_54,puVar2);
          local_4._0_1_ = 4;
          FUN_00619730();
          FUN_005f58b0(local_1c);
          local_24 = 0x90;
          local_20 = 0x98;
          if ((local_54 >> 0x18 < 0x90) || (0x97 < local_54 >> 0x18)) {
            bVar5 = false;
          }
          else {
            bVar5 = true;
          }
          FUN_00619730();
          if (bVar5) {
            FUN_004f26d0(&local_4c,&local_54);
            local_48 = local_3c;
          }
        }
      }
    }
  }
  FUN_004f26d0(param_1,&local_4c);
  local_38 = 1;
  local_4._0_1_ = 3;
  FUN_00435610(local_1c);
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

