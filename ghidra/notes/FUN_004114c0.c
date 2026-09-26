
void FUN_004114c0(void)

{
  undefined4 *puVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  undefined4 local_34;
  int local_30;
  uint local_20;
  undefined4 *local_1c;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a657;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x3c);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = FUN_00420bd0(puVar1);
  }
  local_4 = 0xffffffff;
  if (piVar2 != (int *)0x0) {
    iVar4 = 1;
    piVar3 = piVar2 + 9;
    do {
      iVar5 = 0;
      if (iVar4 < 7) {
        FUN_00401860(&local_34,iVar4,1);
        iVar5 = local_10;
        FUN_004118f0(&local_34);
      }
      *piVar3 = iVar5;
      iVar4 = iVar4 + 1;
      piVar3 = piVar3 + 1;
    } while (iVar4 < 6);
    FUN_0051c300(&local_34);
    local_4 = 1;
    FUN_0051c410(&local_34,piVar2);
    if (local_30 != 0) {
      FUN_005970d0(local_1c,local_20,1);
      FUN_00597170();
    }
    (**(code **)*piVar2)(1);
    local_4 = 0xffffffff;
    FUN_0051c350(&local_34);
  }
  ExceptionList = local_c;
  return;
}

