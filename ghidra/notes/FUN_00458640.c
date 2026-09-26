
void __thiscall FUN_00458640(void *this,int *param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  void *this_00;
  uint *puVar6;
  int *piVar7;
  char *pcVar8;
  int iVar9;
  int iVar10;
  uint uVar11;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063057b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(int *)((int)this + 0x170) != 0) {
    ExceptionList = &pvStack_c;
    DestroyWindow(*(HWND *)(*(int *)((int)this + 0x170) + 0x18));
    if (*(undefined4 **)((int)this + 0x170) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0x170))(1);
    }
    *(undefined4 *)((int)this + 0x170) = 0;
  }
  iVar10 = param_1[9];
  iVar1 = *(int *)(iVar10 + 0x1c);
  iVar2 = *(int *)(iVar10 + 0x20);
  iVar3 = *param_1;
  iVar4 = param_1[1];
  iVar9 = *(int *)(iVar10 + 0x14) + iVar3;
  iVar10 = *(int *)(iVar10 + 0x18) + iVar4;
  this_00 = (void *)FUN_00618b70(0xf4);
  uStack_4 = 0;
  if (this_00 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    uVar5 = *(undefined4 *)((int)this + 0x1c);
    uVar11 = 0;
    puVar6 = (uint *)FUN_006073d0(this);
    piVar7 = FUN_00604cf0(this_00,uVar5,iVar9,iVar10,(iVar1 + 2 + iVar3) - iVar9,
                          (iVar2 + iVar4) - iVar10,(int)this,puVar6,uVar11);
  }
  uStack_4 = 0xffffffff;
  *(int **)((int)this + 0x170) = piVar7;
  if (piVar7 != (int *)0x0) {
    (**(code **)(*piVar7 + 0x18))(10);
    FUN_005ffce0(*(void **)((int)this + 0x170),0);
    pcVar8 = (char *)FUN_00601b20(param_1[9]);
    FUN_00604f90(*(void **)((int)this + 0x170),pcVar8);
    FUN_00605110(*(void **)((int)this + 0x170),0,-1);
    SetFocus(*(HWND *)(*(int *)((int)this + 0x170) + 0x18));
  }
  ExceptionList = pvStack_c;
  return;
}

