
uint * __fastcall FUN_005fb500(int param_1)

{
  undefined2 uVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  void *this;
  undefined2 extraout_var;
  char cVar5;
  uint uVar6;
  undefined4 *puVar7;
  uint *local_78;
  undefined1 local_70 [100];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655e0b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_005f7cc0();
  cVar5 = '\x01';
  uVar6 = 0;
  if ('\0' < *(char *)(param_1 + 0x1d)) {
    do {
      iVar3 = FUN_005f5500((void *)(iVar2 + 0xc),cVar5 + -1 + *(int *)(param_1 + 0x20));
      uVar6 = uVar6 + *(ushort *)(iVar3 + 0x24);
      cVar5 = cVar5 + '\x01';
    } while (cVar5 <= *(char *)(param_1 + 0x1d));
  }
  if (0x1fe < uVar6) {
    FUN_00618ed0(local_70,s_Long_Message__u_006ac784);
    FUN_005f7cc0();
    FUN_005ae1e0();
  }
  local_78 = (uint *)FUN_00618b70(8);
  uVar4 = 0;
  local_4 = 0;
  if (local_78 == (uint *)0x0) {
    local_78 = (uint *)0x0;
  }
  else {
    *local_78 = uVar6;
    if (uVar6 != 0) {
      uVar4 = FUN_00618b70(uVar6);
    }
    local_78[1] = uVar4;
  }
  cVar5 = '\x01';
  local_4 = 0xffffffff;
  puVar7 = (undefined4 *)local_78[1];
  if ('\0' < *(char *)(param_1 + 0x1d)) {
    do {
      this = (void *)FUN_005f5500((void *)(iVar2 + 0xc),cVar5 + -1 + *(int *)(param_1 + 0x20));
      uVar1 = FUN_005fb810(this,puVar7,(ushort)uVar6);
      puVar7 = (undefined4 *)((int)puVar7 + CONCAT22(extraout_var,uVar1));
      uVar6 = uVar6 - CONCAT22(extraout_var,uVar1);
      cVar5 = cVar5 + '\x01';
    } while (cVar5 <= *(char *)(param_1 + 0x1d));
  }
  ExceptionList = local_c;
  return local_78;
}

