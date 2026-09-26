
int __thiscall FUN_005d5b10(void *this,ushort *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  int *unaff_ESI;
  int *piVar3;
  int local_1c;
  undefined4 local_18;
  uint local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065481b;
  local_c = ExceptionList;
  local_1c = 0;
  ExceptionList = &local_c;
  iVar1 = FUN_005c1070();
  if (iVar1 != 0) {
    local_18 = FUN_005fefd0(6);
    local_14 = (uint)*param_1;
    local_10 = 0x12d;
    iVar1 = FUN_005c1070();
    (**(code **)(**(int **)(iVar1 + 0x128) + 0x18))(*(int **)(iVar1 + 0x128),&local_1c);
    piVar3 = (int *)0x1;
    (**(code **)(*unaff_ESI + 0x2c))(unaff_ESI,&stack0xffffffe0,0,1,FUN_005d5a70,this);
    (**(code **)(*piVar3 + 0x34))(piVar3,DAT_0066d2b0,DAT_0066d2b0,DAT_0066d2b0);
    if (local_1c != 0) {
      puVar2 = (undefined4 *)FUN_00618b70(0x20);
      uStack_4 = 0;
      if (puVar2 == (undefined4 *)0x0) {
        puVar2 = (undefined4 *)0x0;
      }
      else {
        FUN_005f5c10(puVar2);
        *(ushort *)(puVar2 + 6) = *param_1;
        puVar2[7] = local_1c;
        *puVar2 = &PTR_FUN_0066d2b4;
      }
      uStack_4 = 0xffffffff;
      FUN_005f4f10((void *)((int)this + 4),(int)puVar2);
    }
  }
  ExceptionList = local_c;
  return local_1c;
}

