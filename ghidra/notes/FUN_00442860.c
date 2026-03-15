
void * __cdecl
FUN_00442860(int param_1,undefined4 param_2,void *param_3,int *param_4,void *param_5,int param_6,
            undefined4 param_7,undefined4 param_8)

{
  void *pvVar1;
  int *piVar2;
  int iVar3;
  void *pvVar4;
  void *this;
  int local_28;
  uint local_24;
  int local_20 [3];
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  iVar3 = param_6;
  pvVar1 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062eb5b;
  local_c = ExceptionList;
  this = (void *)0x0;
  if ((param_3 != (void *)0x0) && (param_6 != 0)) {
    ExceptionList = &local_c;
    FUN_00442a80(param_1,param_2,(int)param_3,&param_1,&local_28,&local_24,local_20);
    pvVar4 = param_5;
    piVar2 = param_4;
    if (*(int *)(iVar3 + 0x3c) < local_28 + local_20[0]) {
      local_28 = local_28 - local_20[0];
    }
    if (param_1 < 0) {
      param_1 = param_1 + local_24;
      if (param_5 != (void *)0x0) {
        param_1 = param_1 + *(int *)((int)param_5 + 0x30);
      }
    }
    else if ((*(int *)(iVar3 + 0x38) < (int)(local_24 + param_1)) &&
            (param_1 = param_1 - local_24, param_5 != (void *)0x0)) {
      param_1 = param_1 - *(int *)((int)param_5 + 0x30);
    }
    pvVar1 = *(void **)((int)pvVar1 + 8);
    if (pvVar1 != (void *)0x0) {
      param_5 = (void *)FUN_00618b70(0x154);
      this = (void *)0x0;
      local_4 = 0;
      if (param_5 != (void *)0x0) {
        this = FUN_004422f0(param_5,piVar2,param_1,local_28,local_24,local_20[0],pvVar4,param_6,
                            param_7,*(undefined4 *)((int)pvVar1 + 0xc),param_8);
      }
      local_4 = 0xffffffff;
      if (this != (void *)0x0) {
        FUN_005fffb0(this,s_Game_Menu_Window_006a8718);
        FUN_005ffce0(this,0);
        FUN_00607740(this,piVar2,0x2779,0x277a,0x277b,(int *)0x2778,(int *)0x2775,(int *)0x2774,
                     0x2777,(int *)0x2776);
      }
    }
    while ((pvVar1 != (void *)0x0 && (this != (void *)0x0))) {
      iVar3 = FUN_005f5ac0(param_3,(int)pvVar1);
      if (iVar3 != 0) {
        FUN_004ab370(this,pvVar1);
        local_14 = *(undefined4 *)((int)pvVar1 + 0x68);
        local_10 = *(undefined4 *)((int)pvVar1 + 0x6c);
        pvVar4 = FUN_00442860(*(int *)((int)pvVar1 + 0x60),*(undefined4 *)((int)pvVar1 + 100),
                              (void *)((int)pvVar1 + 0x80),piVar2,this,param_6,param_7,param_8);
        if (pvVar4 != (void *)0x0) {
          iVar3 = FUN_00401060();
          FUN_004ab3a0(this,piVar2,*(int *)((int)pvVar1 + 0xc),(int)pvVar4,
                       (-(uint)(iVar3 != 1) & 0xb) + 0x2785,(-(uint)(iVar3 != 1) & 0xb) + 0x2786);
        }
      }
      pvVar1 = *(void **)((int)param_3 + 8);
    }
  }
  ExceptionList = local_c;
  return this;
}

