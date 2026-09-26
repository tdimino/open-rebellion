
void __fastcall FUN_00494df0(int param_1)

{
  bool bVar1;
  void **ppvVar2;
  int iVar3;
  uint *puVar4;
  int *piVar5;
  int *this;
  undefined1 auStack_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00635f88;
  if (*(int *)(param_1 + 0x94) == 0) {
    this = (int *)0x0;
  }
  else {
    this = *(int **)(*(int *)(param_1 + 0x94) + 8);
  }
  bVar1 = true;
  ppvVar2 = &local_c;
  local_c = ExceptionList;
  for (; ExceptionList = ppvVar2, this != (int *)0x0; this = (int *)this[2]) {
    iVar3 = (**(code **)(*this + 0xc))();
    if (iVar3 == 0x370) {
      puVar4 = FUN_00494ed0(this,auStack_10);
      uStack_4 = 0;
      piVar5 = FUN_004f3220(*(int *)(param_1 + 0x20),puVar4);
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (piVar5 != (int *)0x0) {
        if (bVar1) {
          FUN_0060b9d0((void *)(param_1 + 0x50),0x710e,(undefined4 *)0x0,(undefined4 *)0x0,
                       (undefined4 *)0x0,(undefined4 *)0x0);
          bVar1 = false;
        }
        FUN_0060b9d0((void *)(param_1 + 0x50),0x710f,piVar5 + 0xc,(undefined4 *)0x0,
                     (undefined4 *)0x0,(undefined4 *)0x0);
      }
    }
    ppvVar2 = ExceptionList;
  }
  ExceptionList = local_c;
  return;
}

