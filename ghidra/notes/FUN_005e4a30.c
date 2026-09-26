
void __fastcall FUN_005e4a30(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  void *pvVar3;
  uint *puVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065501b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar2 = FUN_006037f0(6);
  pvVar3 = (void *)FUN_00618b70(0x118);
  local_4 = 0;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    uVar7 = 0x20;
    uVar6 = 0x20;
    uVar5 = 0xdd07;
    puVar4 = (uint *)FUN_005ff440(param_1);
    pvVar3 = FUN_00607ea0(pvVar3,uVar2,0xd,0x7e,0x7c,0x2d,param_1,puVar4,uVar5,uVar6,uVar7);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xb8) = pvVar3;
  if (pvVar3 != (void *)0x0) {
    FUN_00595b30(pvVar3);
    *(undefined4 *)(*(int *)(param_1 + 0xb8) + 0xf4) = 0x230000;
    FUN_005ffce0(*(void **)(param_1 + 0xb8),0);
    iVar1 = *(int *)(param_1 + 0xb8);
    uVar2 = *(undefined4 *)(param_1 + 0x1c);
    uVar5 = *(undefined4 *)(iVar1 + 0x38);
    puVar4 = (uint *)FUN_005ff440(param_1);
    FUN_0060a560((void *)(iVar1 + 0x94),uVar2,iVar1,6,0x546,uVar5,0,uVar5,0,puVar4);
  }
  ExceptionList = local_c;
  return;
}

