
bool FUN_00545e90(uint *param_1,undefined4 param_2,void *param_3)

{
  bool bVar1;
  int *this;
  int iVar2;
  uint uVar3;
  int *piVar4;
  undefined3 extraout_var;
  bool bVar5;
  undefined4 local_18;
  undefined4 local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647648;
  local_c = ExceptionList;
  bVar5 = true;
  local_18 = 0x30;
  local_14 = 0x40;
  if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    this = (int *)FUN_00505d40(param_1);
    bVar5 = this != (int *)0x0;
    if (this != (int *)0x0) {
      iVar2 = thunk_FUN_00506e60();
      bVar5 = iVar2 != 0;
      if ((iVar2 != 0) && ((*(byte *)(this + 0x1e) & 0x80) == 0)) {
        uVar3 = FUN_00556710(this,param_3);
        if ((uVar3 == 0) || (!bVar5)) {
          bVar5 = false;
        }
        else {
          bVar5 = true;
        }
        if ((*(byte *)(this + 0x14) & 0x40) != 0) {
          local_4 = 0;
          piVar4 = FUN_00402d80(this,&local_18);
          iVar2 = *piVar4;
          FUN_00619730();
          local_4 = 0xffffffff;
          FUN_00619730();
          if (iVar2 == 0x3c000009) {
            iVar2 = FUN_004f71d0(this,0x14,param_3);
            if ((iVar2 == 0) || (!bVar5)) {
              bVar5 = false;
            }
            else {
              bVar5 = true;
            }
          }
        }
      }
    }
    bVar1 = FUN_005438a0((int *)param_1);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      iVar2 = FUN_00549910(local_10,param_3);
      if ((iVar2 == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
  }
  ExceptionList = local_c;
  return bVar5;
}

