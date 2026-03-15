
void * __thiscall
FUN_005d9eb0(void *this,undefined4 param_1,int *param_2,int *param_3,undefined4 param_4,int param_5,
            int param_6,int param_7,int param_8)

{
  int *piVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined4 unaff_EBX;
  undefined4 unaff_EBP;
  undefined4 unaff_ESI;
  undefined4 unaff_retaddr;
  undefined4 *puVar4;
  
  *(undefined4 *)((int)this + 0x10) = param_1;
  piVar1 = (int *)((int)this + 4);
  *(undefined ***)this = &PTR_FUN_0066d490;
  *piVar1 = 0;
  *(undefined4 *)((int)this + 0xc) = param_4;
  if (param_5 == param_6) {
    (**(code **)(*param_3 + 0x6c))(param_3);
  }
  if (param_7 == param_8) {
    (**(code **)(*param_3 + 0x68))(param_3);
  }
  puVar2 = (undefined4 *)((int)this + 8);
  puVar4 = puVar2;
  iVar3 = (**(code **)(*param_2 + 0x10))(param_2,*(undefined4 *)((int)this + 0xc),puVar2);
  *(int *)((int)this + 0x2c) = iVar3;
  if (iVar3 != 0) {
    FUN_005d93f0(*(int *)((int)this + 0x10),iVar3);
  }
  iVar3 = (**(code **)(*(int *)*puVar2 + 0xf4))
                    ((int *)*puVar2,*(undefined4 *)((int)this + 0xc),0,0,0);
  *(int *)((int)this + 0x2c) = iVar3;
  if (iVar3 != 0) {
    FUN_005d93f0(*(int *)((int)this + 0x10),iVar3);
  }
  iVar3 = (**(code **)(*param_2 + 0x50))
                    (param_2,puVar4,*puVar2,unaff_ESI,unaff_retaddr,unaff_EBP,unaff_EBX,piVar1);
  *(int *)((int)this + 0x2c) = iVar3;
  if (iVar3 != 0) {
    FUN_005d93f0(*(int *)((int)this + 0x10),iVar3);
  }
  FUN_005d9420(*(void **)((int)this + 0x10),(int *)*piVar1);
  FUN_005d9420(*(void **)((int)this + 0x10),(int *)*puVar2);
  piVar1 = (int *)*piVar1;
  if (piVar1 != (int *)0x0) {
    iVar3 = (**(code **)(*piVar1 + 0x3c))(piVar1,0x459c4000);
    *(int *)((int)this + 0x2c) = iVar3;
    if (iVar3 != 0) {
      FUN_005d93f0(*(int *)((int)this + 0x10),iVar3);
    }
  }
  return this;
}

