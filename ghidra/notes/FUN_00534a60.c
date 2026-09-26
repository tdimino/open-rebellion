
uint __thiscall FUN_00534a60(void *this,void *param_1)

{
  void *pvVar1;
  bool bVar2;
  undefined3 extraout_var;
  void *this_00;
  uint *puVar3;
  int iVar4;
  int *piVar5;
  uint uVar6;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645a28;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar2 = FUN_004f29b0();
  uVar6 = (uint)(CONCAT31(extraout_var,bVar2) != 0);
  this_00 = (void *)FUN_00539fd0(this,0);
  puVar3 = FUN_0042d170(this_00,&param_1);
  local_4 = 0;
  iVar4 = FUN_004ece60(puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (iVar4 != 0) {
    piVar5 = (int *)FUN_004ece30(&param_1);
    local_4 = 1;
    uVar6 = FUN_00534230(this_00,piVar5,pvVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  FUN_004fd450(&local_28,(int)pvVar1);
  local_4 = 2;
  FUN_004fd620(&local_28,1,1);
  FUN_004fd620(&local_28,2,1);
  local_24 = 1;
  local_20 = 3;
  FUN_00535400(this,&local_28);
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_28);
  ExceptionList = local_c;
  return uVar6;
}

