
void __fastcall FUN_00477950(void *param_1)

{
  int iVar1;
  bool bVar2;
  short sVar3;
  void *this;
  undefined4 *puVar4;
  void *pvVar5;
  uint *puVar6;
  undefined1 local_18 [4];
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006330d0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (void *)thunk_FUN_005f5060((int)param_1 + 0x2c);
  while (this != (void *)0x0) {
    bVar2 = false;
    puVar4 = FUN_00403040(this,local_18);
    iVar1 = *(int *)((int)param_1 + 0x58);
    local_10 = &stack0xffffffd4;
    local_4 = 0;
    FUN_004f26d0(&stack0xffffffd4,puVar4);
    pvVar5 = FUN_00419e40((void *)(iVar1 + 0x78));
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((((pvVar5 != (void *)0x0) && (*(int *)((int)pvVar5 + 0x30) == *(int *)((int)param_1 + 0x18))
         ) && ((*(byte *)((int)pvVar5 + 0x38) & 1) != 0)) &&
       (sVar3 = FUN_005f50e0((int)pvVar5 + 0x48), sVar3 != 0)) {
      bVar2 = true;
    }
    if (bVar2) {
      this = *(void **)((int)this + 0x10);
    }
    else {
      pvVar5 = *(void **)((int)this + 0x10);
      puVar6 = FUN_00403040(this,local_14);
      local_4 = 1;
      FUN_004791d0(param_1,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      this = pvVar5;
    }
  }
  ExceptionList = local_c;
  return;
}

