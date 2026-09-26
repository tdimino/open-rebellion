
int FUN_0051ab20(void *param_1)

{
  bool bVar1;
  void *this;
  uint uVar2;
  int iVar3;
  int iVar4;
  uint local_20;
  undefined4 local_1c;
  undefined4 local_18;
  uint local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642e60;
  local_c = ExceptionList;
  iVar4 = 1;
  ExceptionList = &local_c;
  this = (void *)FUN_00506180();
  do {
    if (this == (void *)0x0) {
      ExceptionList = local_c;
      return iVar4;
    }
    uVar2 = FUN_005092f0((int)this);
    if (uVar2 != 0) {
      uVar2 = *(uint *)((int)this + 0x24) >> 6 & 3;
      if ((uVar2 == 1) || (uVar2 == 2)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      if ((bVar1) && (iVar4 != 0)) {
        iVar4 = 1;
      }
      else {
        iVar4 = 0;
      }
      if (bVar1) {
        FUN_0042dbe0(&local_20);
        local_4 = 1;
        local_4._1_3_ = 0;
        if (uVar2 == 1) {
          local_18 = 0x10000002;
          local_4._0_1_ = 2;
          FUN_004f26d0(&local_20,&local_18);
LAB_0051abec:
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
        }
        else if (uVar2 == 2) {
          local_1c = 0x10000008;
          local_4._0_1_ = 3;
          FUN_004f26d0(&local_20,&local_1c);
          goto LAB_0051abec;
        }
        if (uVar2 == 1) {
          iVar3 = 1;
        }
        else if (uVar2 == 2) {
          iVar3 = 2;
        }
        else {
          iVar3 = 0;
        }
        iVar3 = FUN_00507270(this,iVar3);
        iVar3 = FUN_0055a050(iVar3);
        if (iVar3 != 0) {
          iVar3 = FUN_004f7cf0(this,iVar3,&local_20,uVar2,param_1,1,1);
          if ((iVar3 == 0) || (iVar4 == 0)) {
            iVar4 = 0;
          }
          else {
            iVar4 = 1;
          }
        }
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
    local_14 = 0x90;
    local_10 = 0x92;
    local_4 = 0;
    this = (void *)FUN_004f6010(this,&local_14,1);
    local_4 = 0xffffffff;
    FUN_00619730();
  } while( true );
}

