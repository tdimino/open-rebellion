
void __thiscall FUN_00486d80(void *this,uint *param_1,uint *param_2,undefined4 *param_3)

{
  bool bVar1;
  uint *puVar2;
  int *this_00;
  uint *puVar3;
  int iVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006349d0;
  local_c = ExceptionList;
  if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    this_00 = FUN_004f3110(*(int *)((int)this + 0x24),puVar2);
    if (this_00 != (int *)0x0) {
      puVar3 = (uint *)FUN_0042d170(this_00,&param_1);
      local_4 = 0;
      iVar4 = FUN_004ece60(puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((iVar4 != 0) && ((this_00[0x1e] & 0x100U) == 0)) {
        FUN_0042d170(this_00,&param_1);
        local_4 = 1;
        FUN_0041d4d0(puVar2,(int *)0x0,(uint *)&param_1,(int *)0x0);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  puVar3 = param_2;
  FUN_0041d4d0(puVar2,(int *)0x0,param_2,(int *)0x0);
  FUN_0041d4b0(puVar2,(int *)0x0);
  FUN_0049d3b0((void *)(*(int *)((int)this + 0xc0) + 0x3c),puVar2,puVar3,param_3);
  ExceptionList = local_c;
  return;
}

