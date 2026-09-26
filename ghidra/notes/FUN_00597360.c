
void FUN_00597360(void)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 local_80 [29];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650fa8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00597340();
  uVar5 = 0;
  uVar4 = 0;
  uVar3 = 0;
  uVar1 = FUN_00597080();
  FUN_005faa80(local_80,4,uVar1,uVar3,uVar4,uVar5);
  local_4 = 0;
  puVar2 = (undefined4 *)FUN_005faaf0((int)local_80);
  FUN_005970d0(puVar2,0x74,0);
  FUN_00597170();
  FUN_00618b60((undefined *)puVar2);
  local_4 = 0xffffffff;
  FUN_005faad0(local_80);
  ExceptionList = local_c;
  return;
}

