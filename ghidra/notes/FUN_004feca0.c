
bool __thiscall FUN_004feca0(void *this,uint *param_1,void *param_2)

{
  bool bVar1;
  void *this_00;
  int iVar2;
  uint uVar3;
  bool bVar4;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006405f0;
  local_c = ExceptionList;
  bVar4 = true;
  if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    FUN_004fcd00(local_2c,this,4);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      this_00 = (void *)FUN_0052bed0((int)local_2c);
      FUN_005130d0((int)local_2c);
      if (((*(uint *)((int)this_00 + 0x50) & 0x40) != 0) &&
         ((*(uint *)((int)this_00 + 0x50) & 4) != 0)) {
        iVar2 = FUN_004f8630(this_00,param_1,param_2);
        if ((iVar2 == 0) || (!bVar4)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
      }
    }
    local_4 = 0xffffffff;
    FUN_004fcd80(local_2c);
    ExceptionList = local_c;
    return bVar4;
  }
  uVar3 = *param_1 >> 0x18;
  bVar4 = true;
  local_4 = 1;
  if ((uVar3 < 0x90) || (0x97 < uVar3)) {
    if ((uVar3 < 0xf2) || (0xf2 < uVar3)) {
      bVar4 = false;
    }
    FUN_00619730();
    if (!bVar4) {
      bVar4 = false;
      goto LAB_004feddc;
    }
  }
  bVar4 = true;
LAB_004feddc:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar4) {
    iVar2 = FUN_004f8630(this,param_1,param_2);
    bVar4 = iVar2 != 0;
  }
  else {
    bVar4 = false;
  }
  ExceptionList = local_c;
  return bVar4;
}

