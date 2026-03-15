
int __thiscall FUN_004ed750(void *this,int param_1,void *param_2)

{
  void *this_00;
  int iVar1;
  uint uVar2;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_2;
  puStack_8 = &LAB_0063edd8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = FUN_004f9860(this,param_1,this_00);
  if (iVar1 != 0) {
    if ((*(int *)((int)this_00 + 4) == -1) && ((*(byte *)((int)this + 0x78) & 0x80) != 0)) {
      local_14 = 0x30;
      local_10 = 1;
      local_4 = 1;
      FUN_00520580(this_00,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    if (iVar1 != 0) {
      if ((*(int *)((int)this_00 + 4) == -1) && (*(short *)((int)this + 0x94) != 0)) {
        local_14 = 0x30;
        local_10 = 3;
        local_4 = 2;
        FUN_00520580(this_00,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      if (((iVar1 != 0) && (*(int *)((int)this_00 + 4) == -1)) &&
         ((*(byte *)((int)this + 0xac) & 1) != 0)) {
        local_14 = 0x30;
        local_10 = 2;
        local_4 = 3;
        FUN_00520580(this_00,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  param_2 = (void *)0x0;
  if (((iVar1 != 0) && (iVar1 = FUN_0053ebb0(*(undefined4 *)(param_1 + 0xc),&param_2), iVar1 != 0))
     && (*(int *)((int)this_00 + 4) == -1)) {
    uVar2 = 1;
    switch(param_2) {
    case (void *)0x0:
      uVar2 = (uint)(*(short *)((int)this + 0x96) != 0);
      break;
    case (void *)0x1:
      uVar2 = FUN_004ed1e0((int)this);
      break;
    case (void *)0x2:
      uVar2 = FUN_004ed1c0((int)this);
      break;
    case (void *)0x3:
      uVar2 = FUN_004ed200((int)this);
      break;
    default:
      iVar1 = 0;
    }
    if ((iVar1 != 0) && (uVar2 == 0)) {
      local_14 = 0x30;
      local_10 = 4;
      local_4 = 4;
      FUN_00520580(this_00,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return iVar1;
}

