
uint __cdecl FUN_0053fb00(uint param_1,void *param_2,undefined4 *param_3,void *param_4)

{
  uint *puVar1;
  int iVar2;
  undefined4 *puVar3;
  uint uVar4;
  bool bVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006469f8;
  local_c = ExceptionList;
  puVar3 = (undefined4 *)0x0;
  if (((int)param_1 < 0x380) || (0x396 < (int)param_1)) {
    bVar5 = false;
    ExceptionList = &local_c;
  }
  else {
    ExceptionList = &local_c;
    puVar3 = (undefined4 *)FUN_0054f100(param_1);
    bVar5 = puVar3 != (undefined4 *)0x0;
  }
  uVar4 = 0;
  if (bVar5) {
    uVar4 = puVar3[6];
    if (uVar4 == 0) {
      if (puVar3 != (undefined4 *)0x0) {
        (**(code **)*puVar3)(1);
      }
    }
    else if (puVar3 != (undefined4 *)0x0) {
      puVar1 = FUN_004025b0(param_2,&param_1);
      local_4 = 0;
      FUN_004f26d0(puVar3 + 0xf,puVar1);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005401e0(puVar3 + 0x11,param_3);
      FUN_004fd490(puVar3 + 8,param_4);
      iVar2 = FUN_00586130((int)puVar3);
      FUN_004fd350(puVar3,iVar2);
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

