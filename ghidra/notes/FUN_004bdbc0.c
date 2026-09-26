
bool __fastcall FUN_004bdbc0(int param_1)

{
  uint *puVar1;
  uint *puVar2;
  bool bVar3;
  void *this;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639e08;
  local_c = ExceptionList;
  puVar1 = (uint *)(param_1 + 0x34);
  ExceptionList = &local_c;
  FUN_004ece80(puVar1);
  puVar2 = (uint *)(param_1 + 0x30);
  FUN_004ece80(puVar2);
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xffffffef;
  if (*(uint *)(param_1 + 0x70) != 0) {
    this = (void *)FUN_005f5500((void *)(*(int *)(param_1 + 0x68) + 0xd8),*(uint *)(param_1 + 0x70))
    ;
    if (this != (void *)0x0) {
      FUN_00476e80(this,*(uint *)(param_1 + 0x2c) & 0xffffff,puVar1,puVar2);
    }
  }
  local_4 = 0;
  if ((*puVar2 >> 0x18 != 0) && (*puVar2 >> 0x18 < 0xff)) {
    if ((*puVar1 >> 0x18 < 0x90) || (0x97 < *puVar1 >> 0x18)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      bVar3 = true;
      goto LAB_004bdc92;
    }
  }
  bVar3 = false;
LAB_004bdc92:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar3) {
    *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
  }
  ExceptionList = local_c;
  return bVar3;
}

