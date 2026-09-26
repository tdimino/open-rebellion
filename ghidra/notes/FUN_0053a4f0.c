
uint __thiscall FUN_0053a4f0(void *this,int *param_1,int *param_2)

{
  int iVar1;
  undefined4 *puVar2;
  undefined3 extraout_var;
  uint uVar3;
  uint uVar4;
  bool bVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006462bf;
  local_c = ExceptionList;
  uVar3 = 1;
  ExceptionList = &local_c;
  if (*(int *)((int)this + 0x28) == 0) {
    ExceptionList = &local_c;
    puVar2 = (undefined4 *)FUN_00618b70(0x10);
    if (puVar2 == (undefined4 *)0x0) {
      FUN_00619730();
    }
    local_4 = 0;
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      puVar2 = FUN_00584150(puVar2);
    }
    *(undefined4 **)((int)this + 0x28) = puVar2;
    bVar5 = true;
    uVar3 = 0;
    if (puVar2 == (undefined4 *)0x0) goto LAB_0053a564;
    uVar3 = puVar2[3];
  }
  bVar5 = uVar3 == 0;
LAB_0053a564:
  local_4 = 0xffffffff;
  if (!bVar5) {
    uVar3 = FUN_0053a030(this,param_1);
  }
  iVar1 = param_1[7];
  *param_2 = iVar1;
  if (iVar1 != 0) {
    bVar5 = FUN_005841f0(*(void **)(iVar1 + 0x28),param_1);
    uVar3 = CONCAT31(extraout_var,bVar5);
  }
  uVar4 = 0;
  if (uVar3 != 0) {
    uVar3 = FUN_005f5440(*(void **)((int)this + 0x28),param_1);
    uVar4 = (uint)(uVar3 != 0);
    if (uVar4 != 0) {
      FUN_0060f5d0(param_1,this);
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

