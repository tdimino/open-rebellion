
void __fastcall FUN_0044a210(int param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  undefined *puVar4;
  char *pcVar5;
  undefined4 local_8;
  undefined4 local_4;
  
  iVar2 = FUN_004ece60((uint *)(param_1 + 0x124));
  if (iVar2 == 0) {
    local_8 = CONCAT22(DAT_0065d424,0x3114);
    FUN_006019a0((void *)(param_1 + 0x198),&local_8);
    return;
  }
  local_8 = 0xf1;
  uVar3 = *(uint *)(param_1 + 0x124) >> 0x18;
  local_4 = 0xf2;
  if ((uVar3 < 0xf1) || (0xf1 < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    local_8 = CONCAT22(DAT_0065d424,0x3113);
    FUN_006019a0((void *)(param_1 + 0x198),&local_8);
    return;
  }
  puVar4 = FUN_004f62d0(*(int *)(param_1 + 300));
  pcVar5 = (char *)FUN_00583c40((int)puVar4);
  FUN_00601aa0((void *)(param_1 + 0x198),pcVar5);
  return;
}

