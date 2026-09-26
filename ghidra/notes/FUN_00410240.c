
void __thiscall FUN_00410240(void *this,int param_1)

{
  int iVar1;
  undefined4 *puVar2;
  int *piVar3;
  int iVar4;
  undefined2 extraout_var;
  undefined2 local_5c;
  undefined2 local_5a;
  undefined4 local_58 [3];
  undefined4 local_4c [3];
  undefined4 local_40 [3];
  undefined4 local_34 [3];
  undefined4 local_28;
  int local_24;
  uint local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a537;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f35b0(local_34,&DAT_006a8174);
  local_4 = 0;
  FUN_005f2f50(local_58);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005f35e0(local_58,&DAT_006a8170);
  if (param_1 == 0) {
    if (*(int *)((int)this + 0x17c) == 0) {
      iVar4 = 0;
    }
    else {
      iVar4 = *(int *)((int)this + 0x17c) + 0x98;
    }
    FUN_005f3930(local_58,iVar4,0);
    if (*(void **)((int)this + 0x17c) != (void *)0x0) {
      FUN_00605070(*(void **)((int)this + 0x17c));
    }
  }
  else {
    FUN_005f3930(local_58,param_1,0);
  }
  local_5a = *(undefined2 *)((int)this + 0x94);
  local_5c = 0x1157;
  FUN_005f2fc0(local_40,&local_5c);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_005f3930(local_58,(int)local_40,0);
  iVar4 = 0;
  switch(*(int *)((int)this + 0x11c) == 0) {
  case false:
    iVar4 = *(int *)((int)this + 0x174);
    break;
  case true:
    iVar4 = *(int *)((int)this + 0x178);
  }
  iVar1 = iVar4 + 0x98;
  if (iVar4 == 0) {
    iVar1 = param_1;
  }
  FUN_005f3930(local_58,iVar1,0);
  puVar2 = (undefined4 *)FUN_00618b70(0x50);
  if (puVar2 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4._0_1_ = 3;
  if (puVar2 == (undefined4 *)0x0) {
    piVar3 = (int *)0x0;
  }
  else {
    piVar3 = FUN_00420f10(puVar2);
  }
  local_4._0_1_ = 2;
  if (*(int *)((int)this + 0x180) != 0) {
    FUN_005f2f50(local_4c);
    local_4._0_1_ = 4;
    FUN_005f3930(local_58,(int)local_34,0);
    FUN_0040ffd0((int)local_58,local_4c,CONCAT22(extraout_var,*(undefined2 *)((int)this + 0x94)));
    FUN_00420b70(*(void **)((int)this + 0x180),(int)local_4c,0);
    FUN_005f3090(piVar3 + 0x11,(int)local_58);
    if (piVar3 != (int *)0x0) {
      FUN_0051c300(&local_28);
      local_4 = CONCAT31(local_4._1_3_,5);
      FUN_0051c410(&local_28,piVar3);
      if (local_24 != 0) {
        FUN_005970d0(local_10,local_14,1);
        FUN_00597170();
      }
      (**(code **)*piVar3)(1);
      local_4._0_1_ = 4;
      FUN_0051c350(&local_28);
    }
    local_4._0_1_ = 2;
    FUN_005f2ff0(local_4c);
  }
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_40);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_58);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_34);
  ExceptionList = local_c;
  return;
}

