
int __cdecl FUN_00513a50(int *param_1,int *param_2)

{
  int *piVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined4 *puVar3;
  int *this;
  undefined4 uVar4;
  int iVar5;
  int iVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642572;
  pvStack_c = ExceptionList;
  iVar5 = 0;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_0051c010(0x1b);
  DAT_006b2aec = &LAB_0051c040;
  thunk_FUN_00567430();
  thunk_FUN_0058b370();
  FUN_0053f3d0();
  bVar2 = FUN_0053f580(param_1);
  param_2 = (int *)CONCAT31(extraout_var,bVar2);
  DAT_006b2bb0 = (int *)0x0;
  do {
    puVar3 = (undefined4 *)FUN_00618b70(0x1b0);
    if (puVar3 == (undefined4 *)0x0) {
      FUN_00619730();
    }
    local_4 = 0;
    if (puVar3 == (undefined4 *)0x0) {
      this = (int *)0x0;
    }
    else {
      this = FUN_00513240(puVar3);
    }
    local_4 = 0xffffffff;
    if (this != (int *)0x0) {
      if (iVar5 == 0) {
        DAT_006b2bb0 = this;
      }
      (**(code **)(*this + 0x14))(param_1);
      param_2 = (int *)FUN_0053a1e0(this,param_1);
    }
    iVar5 = iVar5 + 1;
  } while (iVar5 < 3);
  iVar5 = FUN_00506e10();
  *piVar1 = iVar5;
  if ((iVar5 == 0) || (param_2 == (int *)0x0)) {
    iVar6 = 0;
  }
  else {
    iVar6 = 1;
  }
  if (iVar5 != 0) {
    param_1 = (int *)0xf2000003;
    local_4 = 1;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xb0) = uVar4;
    FUN_00619730();
    param_1 = (int *)0x20000005;
    local_4 = 2;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xb4) = uVar4;
    FUN_00619730();
    param_1 = (int *)0xf8000006;
    local_4 = 3;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xb8) = uVar4;
    FUN_00619730();
    param_1 = (int *)0xf9000007;
    local_4 = 4;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xbc) = uVar4;
    FUN_00619730();
    param_1 = (int *)0xfa000008;
    local_4 = 5;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xc0) = uVar4;
    FUN_00619730();
    param_1 = (int *)0xf300000f;
    local_4 = 6;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xc4) = uVar4;
    FUN_00619730();
    param_1 = (int *)0xf3000010;
    local_4 = 7;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    *(undefined4 *)(*piVar1 + 200) = uVar4;
    local_4 = 0xffffffff;
    FUN_00619730();
    param_1 = (int *)0x98000440;
    local_4 = 8;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xcc) = uVar4;
    FUN_00619730();
    param_1 = (int *)0x98000441;
    local_4 = 9;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xd0) = uVar4;
    FUN_00619730();
    param_1 = (int *)0x98000442;
    local_4 = 10;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xd4) = uVar4;
    FUN_00619730();
    param_1 = (int *)0x98000480;
    local_4 = 0xb;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xd8) = uVar4;
    FUN_00619730();
    param_1 = (int *)0x98000481;
    local_4 = 0xc;
    uVar4 = FUN_0053efa0((uint *)&param_1);
    local_4 = 0xffffffff;
    *(undefined4 *)(*piVar1 + 0xdc) = uVar4;
    FUN_00619730();
    iVar5 = FUN_0051a530(*piVar1);
    if ((iVar5 == 0) || (iVar6 == 0)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar5 = FUN_00518dc0(*piVar1);
    if ((iVar5 == 0) || (!bVar2)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
  }
  FUN_0051c010(0x1a);
  ExceptionList = pvStack_c;
  return iVar6;
}

