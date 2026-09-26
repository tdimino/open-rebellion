
void __thiscall FUN_004080c0(void *this,int param_1,int param_2)

{
  int iVar1;
  DWORD DVar2;
  bool bVar3;
  undefined1 auStack_24 [8];
  undefined4 uStack_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00629a68;
  pvStack_c = ExceptionList;
  bVar3 = false;
  local_4 = 0;
  if (param_1 == 1) {
    ExceptionList = &pvStack_c;
    iVar1 = FUN_00401080();
    bVar3 = iVar1 == 2;
  }
  else {
    ExceptionList = &pvStack_c;
    if ((param_1 == 2) &&
       ((*(int *)((int)this + 0x9c) == 4 ||
        (ExceptionList = &pvStack_c, *(int *)((int)this + 0x9c) == 5)))) {
      bVar3 = true;
      ExceptionList = &pvStack_c;
    }
  }
  if (bVar3) {
    DVar2 = GetTickCount();
    uStack_1c = 0x408123;
    FUN_00401420(DVar2);
    FUN_005f2f90(auStack_24,(int)&stack0x0000000c);
    FUN_004013a0(param_1,param_2);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x0000000c);
  ExceptionList = pvStack_c;
  return;
}

