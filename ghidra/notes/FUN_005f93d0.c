
bool __thiscall FUN_005f93d0(void *this,int param_1)

{
  char cVar1;
  uint uVar2;
  int iVar3;
  uint uVar4;
  char *pcVar5;
  undefined4 local_22c [8];
  uint local_20c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655beb;
  local_c = ExceptionList;
  uVar2 = *(uint *)(param_1 + 0x20);
  uVar4 = 0xffffffff;
  pcVar5 = (char *)((int)this + 0x114);
  do {
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar1 != '\0');
  ExceptionList = &local_c;
  FUN_005fadf0(local_22c,(undefined4 *)((int)this + 0x114),~uVar4,uVar2,1,1,0);
  local_4 = 0;
  local_20c = uVar2;
  iVar3 = FUN_005f9090(this,(int)local_22c,uVar2 >> 0x1c,0);
  if (iVar3 != 0) {
    FUN_005ae1e0();
    local_4 = 0xffffffff;
    FUN_005facc0(local_22c);
  }
  else {
    local_4 = 0xffffffff;
    FUN_005facc0(local_22c);
  }
  ExceptionList = local_c;
  return iVar3 == 0;
}

