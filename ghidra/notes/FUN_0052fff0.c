
int __thiscall FUN_0052fff0(void *this,void *param_1)

{
  bool bVar1;
  int *this_00;
  int iVar2;
  uint uVar3;
  int *piVar4;
  int iVar5;
  int local_54;
  uint local_34;
  undefined4 local_30;
  int local_2c [2];
  int local_24 [2];
  undefined1 auStack_1c [8];
  undefined1 auStack_14 [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645470;
  pvStack_c = ExceptionList;
  local_54 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520570(local_24);
  local_4 = 0;
  FUN_00520570(local_2c);
  local_4._0_1_ = 1;
  this_00 = (int *)FUN_00505340();
  do {
    if (this_00 == (int *)0x0) {
      iVar5 = FUN_0052ed50(this,local_24,param_1);
      iVar2 = 0;
      if (iVar5 != 0) {
        iVar5 = FUN_0052ee10(this,local_2c,param_1);
        iVar2 = 0;
        if (iVar5 != 0) {
          iVar5 = FUN_0052eed0(this,local_54,param_1);
          iVar2 = 0;
          if (iVar5 != 0) {
            iVar5 = FUN_0052ef30(this,local_54,param_1);
            iVar2 = 0;
            if (iVar5 != 0) {
              iVar5 = FUN_0052f6b0(this,param_1);
              iVar2 = 0;
              if (iVar5 != 0) {
                iVar2 = FUN_0052f8f0(this,param_1);
              }
            }
          }
        }
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = pvStack_c;
      return iVar2;
    }
    if (((*(byte *)(this_00 + 0x14) & 0x40) != 0) &&
       (((this_00[9] ^ *(uint *)((int)this + 0x24)) & 0xc0) == 0)) {
      iVar2 = FUN_004f2990((int)this_00);
      local_54 = local_54 + iVar2;
      local_4._0_1_ = 3;
      uVar3 = (**(code **)(*this_00 + 4))();
      if ((uVar3 < 0x2c) || (0x2f < uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4._0_1_ = 1;
      FUN_00619730();
      if (((bVar1) && ((*(byte *)(this_00 + 0x14) & 1) != 0)) &&
         ((*(byte *)(this_00 + 0x18) & 1) == 0)) {
        local_4._0_1_ = 4;
        uVar3 = (**(code **)(*this_00 + 4))();
        if ((uVar3 < 0x2c) || (0x2c < uVar3)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        local_4._0_1_ = 1;
        FUN_00619730();
        if (bVar1) {
          piVar4 = FUN_0052e740(this_00,auStack_1c);
          local_4._0_1_ = 5;
          FUN_0051c9c0(local_24,piVar4);
        }
        else {
          local_4._0_1_ = 6;
          uVar3 = (**(code **)(*this_00 + 4))();
          if ((uVar3 < 0x2d) || (0x2d < uVar3)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          local_4._0_1_ = 1;
          FUN_00619730();
          if (!bVar1) goto LAB_00530190;
          piVar4 = FUN_0052e740(this_00,auStack_14);
          local_4._0_1_ = 7;
          FUN_0051c9c0(local_2c,piVar4);
        }
        local_4._0_1_ = 1;
        FUN_00619730();
      }
    }
LAB_00530190:
    local_34 = 0x10;
    local_30 = 0x40;
    local_4._0_1_ = 2;
    this_00 = (int *)FUN_004f6010(this_00,&local_34,1);
    local_4._0_1_ = 1;
    FUN_00619730();
  } while( true );
}

