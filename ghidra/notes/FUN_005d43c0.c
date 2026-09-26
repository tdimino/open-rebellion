
void * __thiscall
FUN_005d43c0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,ushort param_4)

{
  int *piVar1;
  HMODULE hModule;
  int iVar2;
  undefined4 uVar3;
  void *pvVar4;
  int iVar5;
  int *piStack_6c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654718;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005d8980(this,param_2,param_1);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066d1f8;
  *(undefined ***)((int)this + 0x1c) = &PTR_LAB_0066d1e8;
  *(undefined4 *)((int)this + 0x3c) = param_3;
  *(undefined4 *)((int)this + 0x4c) = 1;
  hModule = (HMODULE)FUN_005fefd0(6);
  FindResourceA(hModule,(LPCSTR)(uint)param_4,&DAT_0000012f);
  piVar1 = (int *)(DAT_006bcbd4 + 0x168);
  for (iVar2 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
    if (*(ushort *)(iVar2 + 0x18) == param_4) goto LAB_005d4465;
  }
  iVar2 = 0;
LAB_005d4465:
  if (iVar2 == 0) {
    uVar3 = (**(code **)(*piVar1 + 4))();
    *(undefined4 *)((int)this + 0x40) = uVar3;
  }
  else {
    *(undefined4 *)((int)this + 0x40) = *(undefined4 *)(iVar2 + 0x1c);
  }
  iVar2 = (**(code **)(**(int **)((int)this + 0x40) + 0x48))();
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x40) + 0x3c))();
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x40) + 0x40))();
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x40) + 0x50))();
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x40) + 0x44))();
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x40) + 0x18))();
  FUN_005d8b00(iVar2);
  piVar1 = (int *)(DAT_006bcbd4 + 0x168);
  for (iVar2 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
    if (*(short *)(iVar2 + 0x18) == (ushort)(param_4 + 5)) goto LAB_005d4543;
  }
  iVar2 = 0;
LAB_005d4543:
  if (iVar2 == 0) {
    uVar3 = (**(code **)(*piVar1 + 4))();
    *(undefined4 *)((int)this + 0x44) = uVar3;
  }
  else {
    *(undefined4 *)((int)this + 0x44) = *(undefined4 *)(iVar2 + 0x1c);
  }
  iVar2 = (**(code **)(**(int **)((int)this + 0x44) + 0x48))();
  piStack_6c = (int *)0x5d4573;
  FUN_005d8b00(iVar2);
  piStack_6c = *(int **)((int)this + 0x44);
  iVar2 = (**(code **)(*piStack_6c + 0x3c))();
  FUN_005d8b00(iVar2);
  piStack_6c = (int *)0x0;
  iVar2 = (**(code **)(**(int **)((int)this + 0x44) + 0x40))(*(int **)((int)this + 0x44),1);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x44) + 0x50))(*(int **)((int)this + 0x44),1,0);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x44) + 0x44))
                    (*(int **)((int)this + 0x44),0x3e99999a,0x3e99999a,0);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x44) + 0x18))(*(int **)((int)this + 0x44),0,0);
  FUN_005d8b00(iVar2);
  piStack_6c = (int *)((int)(short)param_4 % 10 + 0x1068);
  piVar1 = (int *)(DAT_006bcbd4 + 0x168);
  for (iVar2 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
    if (*(short *)(iVar2 + 0x18) == (short)piStack_6c) goto LAB_005d462f;
  }
  iVar2 = 0;
LAB_005d462f:
  if (iVar2 == 0) {
    uVar3 = (**(code **)(*piVar1 + 4))(&piStack_6c);
    *(undefined4 *)((int)this + 0x48) = uVar3;
  }
  else {
    *(undefined4 *)((int)this + 0x48) = *(undefined4 *)(iVar2 + 0x1c);
  }
  iVar5 = 0;
  iVar2 = (**(code **)(**(int **)((int)this + 0x48) + 0x48))(*(int **)((int)this + 0x48),0,0,0);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x48) + 0x3c))(*(int **)((int)this + 0x48),1,0);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x48) + 0x40))(*(int **)((int)this + 0x48),1,0);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x48) + 0x50))(*(int **)((int)this + 0x48),0,0);
  FUN_005d8b00(iVar2);
  pvVar4 = (void *)0x0;
  iVar2 = (**(code **)(**(int **)((int)this + 0x48) + 0x44))
                    (*(int **)((int)this + 0x48),0x3e99999a,0x3e99999a);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)((int)this + 0x48) + 0x18))(*(int **)((int)this + 0x48),0,0);
  FUN_005d8b00(iVar2);
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  if (iVar5 == 1) {
    FUN_005d4800((int)this);
  }
  ExceptionList = pvVar4;
  return this;
}

