
void __fastcall FUN_0047a480(int *param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00633498;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  param_1[8] = param_1[8] & 0xfffffffd;
  this = (uint *)(param_1 + 10);
  if (((uint)param_1[10] >> 0x18 < 0x90) || (0x97 < (uint)param_1[10] >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    iVar1 = param_1[0x11];
    FUN_004f26d0(&stack0xffffffdc,this);
    pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if (pvVar3 != (void *)0x0) {
      if ((*(uint *)((int)pvVar3 + 0x30) & 0x10000000) == 0) {
        (**(code **)(*param_1 + 0x2c))();
      }
      else {
        param_1[8] = param_1[8] | 2;
      }
    }
  }
  if ((*(byte *)(param_1 + 8) & 2) == 0) {
    FUN_004ece80(this);
    puVar4 = (undefined4 *)(**(code **)(*param_1 + 0x30))();
    uStack_4 = 0;
    FUN_004f26d0(this,puVar4);
    uStack_4 = 0xffffffff;
    FUN_00619730();
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      param_1[8] = param_1[8] | 2;
    }
  }
  ExceptionList = local_c;
  return;
}

