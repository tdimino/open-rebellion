
int FUN_0051b080(void *param_1)

{
  void *this;
  int iVar1;
  int iVar2;
  int *piVar3;
  bool bVar4;
  uint local_24 [6];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00642ef0;
  local_c = ExceptionList;
  local_24[0] = 0x90000109;
  piVar3 = (int *)0x0;
  local_4 = 0;
  ExceptionList = &local_c;
  this = (void *)FUN_00505f60(local_24);
  local_4 = 0xffffffff;
  FUN_00619730();
  bVar4 = this != (void *)0x0;
  if (this != (void *)0x0) {
    local_24[1] = 0x8000004;
    local_4 = 1;
    piVar3 = FUN_004f7d50(this,local_24 + 1,*(uint *)((int)this + 0x24) >> 6 & 3,param_1,1,1);
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((piVar3 == (int *)0x0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_24[4] = 0;
  local_24[3] = 0;
  local_24[2] = 0;
  iVar1 = FUN_0055d5e0(local_24 + 4,local_24 + 3,local_24 + 2);
  if ((iVar1 == 0) || (!bVar4)) {
    iVar1 = 0;
  }
  else {
    iVar1 = 1;
  }
  if (this != (void *)0x0) {
    iVar2 = FUN_0051be90(piVar3,local_24[4],local_24[3],local_24[2],param_1);
    if ((iVar2 == 0) || (iVar1 == 0)) {
      iVar1 = 0;
    }
    else {
      iVar1 = 1;
    }
  }
  ExceptionList = local_c;
  return iVar1;
}

