
void __thiscall FUN_0060da10(void *this,uint *param_1,int param_2,uint param_3)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  void *this_00;
  int iVar3;
  int iVar4;
  uint *puVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656beb;
  local_c = ExceptionList;
  puVar1 = *(undefined4 **)((int)this + 0x98);
  if ((puVar1 != (undefined4 *)0x0) && (param_1 != (uint *)0x0)) {
    ExceptionList = &local_c;
    if (puVar1 != (undefined4 *)0x0) {
      ExceptionList = &local_c;
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    this_00 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (this_00 == (void *)0x0) {
      puVar5 = (uint *)0x0;
    }
    else {
      puVar5 = param_1;
      iVar3 = FUN_005fc0f0((int *)param_1);
      iVar4 = FUN_005fc0e0((int *)param_1);
      puVar5 = FUN_005fbda0(this_00,iVar4,iVar3,puVar5);
    }
    local_4 = 0xffffffff;
    *(uint **)((int)this + 0x98) = puVar5;
    FUN_005fcc30(param_1,puVar5,0,0,param_2,param_3,(undefined4 *)0x0,0);
    for (iVar3 = FUN_005f5060((int)this + 0x6c); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      uVar2 = *(undefined4 *)((int)this + 0x98);
      *(undefined4 *)(iVar3 + 0x10c) = 0;
      *(undefined4 *)(iVar3 + 0xa4) = uVar2;
      *(undefined4 *)(iVar3 + 0x110) = 0;
    }
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  }
  ExceptionList = local_c;
  return;
}

